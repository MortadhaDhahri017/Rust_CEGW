#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <stdint.h>
#include <unistd.h>

#define IOCTL_GET_READ_COUNT 0x80086301
#define IOCTL_SET_READ_COUNT 0x40086301

void get_read_count(int fd) {
    uint64_t read_count;
    if (ioctl(fd, IOCTL_GET_READ_COUNT, &read_count) == -1) {
        perror("ioctl get");
        exit(EXIT_FAILURE);
    }
    printf("Read count: %llu\n", read_count);
}

void set_read_count(int fd, uint64_t count) {
    if (ioctl(fd, IOCTL_SET_READ_COUNT, &count) == -1) {
        perror("ioctl set");
        exit(EXIT_FAILURE);
    }
}

int main() {
    const char *device = "/dev/semaphore";
    int fd = open(device, O_RDWR);
    if (fd == -1) {
        perror("open");
        return EXIT_FAILURE;
    }

    // Get the read count
    get_read_count(fd);

    // Set the read count to 42
    set_read_count(fd, 42);
    printf("Set read count to 42\n");

    // Get the read count again
    get_read_count(fd);

    close(fd);
    return 0;
}
