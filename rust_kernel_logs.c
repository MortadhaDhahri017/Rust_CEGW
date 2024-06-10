#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/init.h>
#include <linux/socket.h>
#include <linux/in.h>
#include <linux/net.h>
#include <linux/slab.h>
#include <linux/kthread.h>
#include <linux/delay.h>
#include <linux/kmsg_dump.h>
#include <linux/string.h>
#include <linux/inet.h>
#include <linux/uaccess.h>

#define SERVER_IP_1 192
#define SERVER_IP_2 168
#define SERVER_IP_3 75
#define SERVER_IP_4 128
#define SERVER_PORT 5700

struct socket *sock;
static struct kmsg_dumper dumper;

static int send_to_user(const char *message)
{
    struct msghdr msg = {0};
    struct kvec iov;
    int len = strlen(message) + 1;
    int ret;

    iov.iov_base = (void *)message;
    iov.iov_len = len;

    ret = kernel_sendmsg(sock, &msg, &iov, 1, len);

    return ret;
}

static int connect_to_user(void)
{
    struct sockaddr_in saddr;
    int ret;

    ret = sock_create(PF_INET, SOCK_STREAM, IPPROTO_TCP, &sock);
    if (ret < 0) {
        printk(KERN_ERR "Failed to create socket\n");
        return ret;
    }

    memset(&saddr, 0, sizeof(saddr));
    saddr.sin_family = AF_INET;
    saddr.sin_port = htons(SERVER_PORT);

    // Convert IP address manually
    saddr.sin_addr.s_addr = htonl((SERVER_IP_1 << 24) | (SERVER_IP_2 << 16) | (SERVER_IP_3 << 8) | SERVER_IP_4);

    ret = kernel_connect(sock, (struct sockaddr *)&saddr, sizeof(saddr), 0);
    if (ret < 0) {
        printk(KERN_ERR "Failed to connect to user space\n");
        sock_release(sock);
        return ret;
    }

    return 0;
}

static void dump_kernel_logs(void)
{
    struct kmsg_dump_iter iter;
    char buffer[1024];
    size_t len;

    while (!kthread_should_stop()) {
        kmsg_dump_rewind(&iter);
        while (kmsg_dump_get_line(&iter, true, buffer, sizeof(buffer), &len)) {
            buffer[len] = '\0';
            send_to_user(buffer);
        }
        msleep(5000);
    }
}

static int kernel_thread_fn(void *data)
{
    dump_kernel_logs();
    return 0;
}

static int __init my_module_init(void)
{
    int ret;
    struct task_struct *task;

    ret = connect_to_user();
    if (ret < 0) {
        printk(KERN_ERR "Could not connect to user space\n");
        return ret;
    }

    kmsg_dump_register(&dumper);

    task = kthread_run(kernel_thread_fn, NULL, "my_kernel_thread");
    if (IS_ERR(task)) {
        printk(KERN_ERR "Failed to create kernel thread\n");
        return PTR_ERR(task);
    }

    return 0;
}

static void __exit my_module_exit(void)
{
    kmsg_dump_unregister(&dumper);
    if (sock) {
        kernel_sock_shutdown(sock, SHUT_RDWR);
        sock_release(sock);
    }
    printk(KERN_INFO "Kernel module exiting\n");
}

module_init(my_module_init);
module_exit(my_module_exit);

MODULE_LICENSE("GPL");