// SPDX-License-Identifier: GPL-2.0
/*
 * Non-trivial C macros cannot be used in Rust. Similarly, inlined C functions
 * cannot be called either. This file explicitly creates functions ("helpers")
 * that wrap those so that they can be called from Rust.
 *
 * Even though Rust kernel modules should never use directly the bindings, some
 * of these helpers need to be exported because Rust generics and inlined
 * functions may not get their code generated in the crate where they are
 * defined. Other helpers, called from non-inline functions, may not be
 * exported, in principle. However, in general, the Rust compiler does not
 * guarantee codegen will be performed for a non-inline function either.
 * Therefore, this file exports all the helpers. In the future, this may be
 * revisited to reduce the number of exports after the compiler is informed
 * about the places codegen is required.
 *
 * All symbols are exported as GPL-only to guarantee no GPL-only feature is
 * accidentally exposed.
 */

#include <linux/amba/bus.h>
#include <linux/bug.h>
#include <linux/build_bug.h>
#include <linux/clk.h>
#include <linux/errname.h>
#include <linux/etherdevice.h>
#include <linux/fs_parser.h>
#include <linux/gfp.h>
#include <linux/highmem.h>
#include <linux/io.h>
#include <linux/irqchip/chained_irq.h>
#include <linux/irqdomain.h>
#include <linux/irq.h>
#include <linux/mutex.h>
#include <linux/netdevice.h>
#include <linux/of_device.h>
#include <linux/pci.h>
#include <linux/platform_device.h>
#include <linux/sched/signal.h>
#include <linux/security.h>
#include <linux/skbuff.h>
#include <linux/uaccess.h>
#include <linux/uio.h>
#include <net/netlink.h>
#include <net/genetlink.h> 
#include <linux/types.h>
#include <linux/list.h>
#include <linux/rculist.h>
#include <linux/can.h> 
#include <linux/can/raw.h>
#include <linux/can/error.h>
#include <linux/can/dev.h> 
#include <linux/can/vxcan.h>
#include <linux/if_ether.h>
#include <linux/can/core.h>
#include <net/ip.h>
#include <linux/can/skb.h>
#include <linux/can/can-ml.h>
#include <linux/ratelimit.h>



__noreturn void rust_helper_BUG(void)
{
	BUG();
}
EXPORT_SYMBOL_GPL(rust_helper_BUG);

void rust_helper_clk_disable_unprepare(struct clk *clk)
{
	return clk_disable_unprepare(clk);
}
EXPORT_SYMBOL_GPL(rust_helper_clk_disable_unprepare);

int rust_helper_clk_prepare_enable(struct clk *clk)
{
	return clk_prepare_enable(clk);
}
EXPORT_SYMBOL_GPL(rust_helper_clk_prepare_enable);

unsigned long rust_helper_copy_from_user(void *to, const void __user *from, unsigned long n)
{
	return copy_from_user(to, from, n);
}
EXPORT_SYMBOL_GPL(rust_helper_copy_from_user);

unsigned long rust_helper_copy_to_user(void __user *to, const void *from, unsigned long n)
{
	return copy_to_user(to, from, n);
}
EXPORT_SYMBOL_GPL(rust_helper_copy_to_user);

unsigned long rust_helper_clear_user(void __user *to, unsigned long n)
{
	return clear_user(to, n);
}
EXPORT_SYMBOL_GPL(rust_helper_clear_user);

void __iomem *rust_helper_ioremap(resource_size_t offset, unsigned long size)
{
	return ioremap(offset, size);
}
EXPORT_SYMBOL_GPL(rust_helper_ioremap);

u8 rust_helper_readb(const volatile void __iomem *addr)
{
	return readb(addr);
}
EXPORT_SYMBOL_GPL(rust_helper_readb);

u16 rust_helper_readw(const volatile void __iomem *addr)
{
	return readw(addr);
}
EXPORT_SYMBOL_GPL(rust_helper_readw);

u32 rust_helper_readl(const volatile void __iomem *addr)
{
	return readl(addr);
}
EXPORT_SYMBOL_GPL(rust_helper_readl);

#ifdef CONFIG_64BIT
u64 rust_helper_readq(const volatile void __iomem *addr)
{
	return readq(addr);
}
EXPORT_SYMBOL_GPL(rust_helper_readq);
#endif

void rust_helper_writeb(u8 value, volatile void __iomem *addr)
{
	writeb(value, addr);
}
EXPORT_SYMBOL_GPL(rust_helper_writeb);

void rust_helper_writew(u16 value, volatile void __iomem *addr)
{
	writew(value, addr);
}
EXPORT_SYMBOL_GPL(rust_helper_writew);

void rust_helper_writel(u32 value, volatile void __iomem *addr)
{
	writel(value, addr);
}
EXPORT_SYMBOL_GPL(rust_helper_writel);

#ifdef CONFIG_64BIT
void rust_helper_writeq(u64 value, volatile void __iomem *addr)
{
	writeq(value, addr);
}
EXPORT_SYMBOL_GPL(rust_helper_writeq);
#endif

u8 rust_helper_readb_relaxed(const volatile void __iomem *addr)
{
	return readb_relaxed(addr);
}
EXPORT_SYMBOL_GPL(rust_helper_readb_relaxed);

u16 rust_helper_readw_relaxed(const volatile void __iomem *addr)
{
	return readw_relaxed(addr);
}
EXPORT_SYMBOL_GPL(rust_helper_readw_relaxed);

u32 rust_helper_readl_relaxed(const volatile void __iomem *addr)
{
	return readl_relaxed(addr);
}
EXPORT_SYMBOL_GPL(rust_helper_readl_relaxed);

#ifdef CONFIG_64BIT
u64 rust_helper_readq_relaxed(const volatile void __iomem *addr)
{
	return readq_relaxed(addr);
}
EXPORT_SYMBOL_GPL(rust_helper_readq_relaxed);
#endif

void rust_helper_writeb_relaxed(u8 value, volatile void __iomem *addr)
{
	writeb_relaxed(value, addr);
}
EXPORT_SYMBOL_GPL(rust_helper_writeb_relaxed);

void rust_helper_writew_relaxed(u16 value, volatile void __iomem *addr)
{
	writew_relaxed(value, addr);
}
EXPORT_SYMBOL_GPL(rust_helper_writew_relaxed);

void rust_helper_writel_relaxed(u32 value, volatile void __iomem *addr)
{
	writel_relaxed(value, addr);
}
EXPORT_SYMBOL_GPL(rust_helper_writel_relaxed);

#ifdef CONFIG_64BIT
void rust_helper_writeq_relaxed(u64 value, volatile void __iomem *addr)
{
	writeq_relaxed(value, addr);
}
EXPORT_SYMBOL_GPL(rust_helper_writeq_relaxed);
#endif

void rust_helper_memcpy_fromio(void *to, const volatile void __iomem *from, long count)
{
	memcpy_fromio(to, from, count);
}
EXPORT_SYMBOL_GPL(rust_helper_memcpy_fromio);

void rust_helper___spin_lock_init(spinlock_t *lock, const char *name,
				  struct lock_class_key *key)
{
#ifdef CONFIG_DEBUG_SPINLOCK
	__spin_lock_init(lock, name, key);
#else
	spin_lock_init(lock);
#endif
}
EXPORT_SYMBOL_GPL(rust_helper___spin_lock_init);

void rust_helper_spin_lock(spinlock_t *lock)
{
	spin_lock(lock);
}
EXPORT_SYMBOL_GPL(rust_helper_spin_lock);

void rust_helper_spin_unlock(spinlock_t *lock)
{
	spin_unlock(lock);
}
EXPORT_SYMBOL_GPL(rust_helper_spin_unlock);

unsigned long rust_helper_spin_lock_irqsave(spinlock_t *lock)
{
	unsigned long flags;

	spin_lock_irqsave(lock, flags);

	return flags;
}
EXPORT_SYMBOL_GPL(rust_helper_spin_lock_irqsave);

void rust_helper_spin_unlock_irqrestore(spinlock_t *lock, unsigned long flags)
{
	spin_unlock_irqrestore(lock, flags);
}
EXPORT_SYMBOL_GPL(rust_helper_spin_unlock_irqrestore);

void rust_helper__raw_spin_lock_init(raw_spinlock_t *lock, const char *name,
				     struct lock_class_key *key)
{
#ifdef CONFIG_DEBUG_SPINLOCK
	_raw_spin_lock_init(lock, name, key);
#else
	raw_spin_lock_init(lock);
#endif
}
EXPORT_SYMBOL_GPL(rust_helper__raw_spin_lock_init);

void rust_helper_raw_spin_lock(raw_spinlock_t *lock)
{
	raw_spin_lock(lock);
}
EXPORT_SYMBOL_GPL(rust_helper_raw_spin_lock);

void rust_helper_raw_spin_unlock(raw_spinlock_t *lock)
{
	raw_spin_unlock(lock);
}
EXPORT_SYMBOL_GPL(rust_helper_raw_spin_unlock);

unsigned long rust_helper_raw_spin_lock_irqsave(raw_spinlock_t *lock)
{
	unsigned long flags;

	raw_spin_lock_irqsave(lock, flags);

	return flags;
}
EXPORT_SYMBOL_GPL(rust_helper_raw_spin_lock_irqsave);

void rust_helper_raw_spin_unlock_irqrestore(raw_spinlock_t *lock,
					    unsigned long flags)
{
	raw_spin_unlock_irqrestore(lock, flags);
}
EXPORT_SYMBOL_GPL(rust_helper_raw_spin_unlock_irqrestore);

void rust_helper_init_wait(struct wait_queue_entry *wq_entry)
{
	init_wait(wq_entry);
}
EXPORT_SYMBOL_GPL(rust_helper_init_wait);

void rust_helper_init_waitqueue_func_entry(struct wait_queue_entry *wq_entry,
					   wait_queue_func_t func)
{
	init_waitqueue_func_entry(wq_entry, func);
}
EXPORT_SYMBOL_GPL(rust_helper_init_waitqueue_func_entry);

int rust_helper_signal_pending(struct task_struct *t)
{
	return signal_pending(t);
}
EXPORT_SYMBOL_GPL(rust_helper_signal_pending);

struct page *rust_helper_alloc_pages(gfp_t gfp_mask, unsigned int order)
{
	return alloc_pages(gfp_mask, order);
}
EXPORT_SYMBOL_GPL(rust_helper_alloc_pages);

void *rust_helper_kmap(struct page *page)
{
	return kmap(page);
}
EXPORT_SYMBOL_GPL(rust_helper_kmap);

void rust_helper_kunmap(struct page *page)
{
	return kunmap(page);
}
EXPORT_SYMBOL_GPL(rust_helper_kunmap);

int rust_helper_cond_resched(void)
{
	return cond_resched();
}
EXPORT_SYMBOL_GPL(rust_helper_cond_resched);

size_t rust_helper_copy_from_iter(void *addr, size_t bytes, struct iov_iter *i)
{
	return copy_from_iter(addr, bytes, i);
}
EXPORT_SYMBOL_GPL(rust_helper_copy_from_iter);

size_t rust_helper_copy_to_iter(const void *addr, size_t bytes, struct iov_iter *i)
{
	return copy_to_iter(addr, bytes, i);
}
EXPORT_SYMBOL_GPL(rust_helper_copy_to_iter);

bool rust_helper_IS_ERR(__force const void *ptr)
{
	return IS_ERR(ptr);
}
EXPORT_SYMBOL_GPL(rust_helper_IS_ERR);

long rust_helper_PTR_ERR(__force const void *ptr)
{
	return PTR_ERR(ptr);
}
EXPORT_SYMBOL_GPL(rust_helper_PTR_ERR);

const char *rust_helper_errname(int err)
{
	return errname(err);
}
EXPORT_SYMBOL_GPL(rust_helper_errname);

void rust_helper_mutex_lock(struct mutex *lock)
{
	mutex_lock(lock);
}
EXPORT_SYMBOL_GPL(rust_helper_mutex_lock);

void rust_helper_amba_set_drvdata(struct amba_device *dev, void *data)
{
	amba_set_drvdata(dev, data);
}
EXPORT_SYMBOL_GPL(rust_helper_amba_set_drvdata);

void *rust_helper_amba_get_drvdata(struct amba_device *dev)
{
	return amba_get_drvdata(dev);
}
EXPORT_SYMBOL_GPL(rust_helper_amba_get_drvdata);

void *
rust_helper_platform_get_drvdata(const struct platform_device *pdev)
{
	return platform_get_drvdata(pdev);
}
EXPORT_SYMBOL_GPL(rust_helper_platform_get_drvdata);

void
rust_helper_platform_set_drvdata(struct platform_device *pdev,
				 void *data)
{
	return platform_set_drvdata(pdev, data);
}
EXPORT_SYMBOL_GPL(rust_helper_platform_set_drvdata);

refcount_t rust_helper_REFCOUNT_INIT(int n)
{
	return (refcount_t)REFCOUNT_INIT(n);
}
EXPORT_SYMBOL_GPL(rust_helper_REFCOUNT_INIT);

void rust_helper_refcount_inc(refcount_t *r)
{
	refcount_inc(r);
}
EXPORT_SYMBOL_GPL(rust_helper_refcount_inc);

bool rust_helper_refcount_dec_and_test(refcount_t *r)
{
	return refcount_dec_and_test(r);
}
EXPORT_SYMBOL_GPL(rust_helper_refcount_dec_and_test);

void rust_helper_rb_link_node(struct rb_node *node, struct rb_node *parent,
			      struct rb_node **rb_link)
{
	rb_link_node(node, parent, rb_link);
}
EXPORT_SYMBOL_GPL(rust_helper_rb_link_node);

struct task_struct *rust_helper_get_current(void)
{
	return current;
}
EXPORT_SYMBOL_GPL(rust_helper_get_current);

void rust_helper_get_task_struct(struct task_struct *t)
{
	get_task_struct(t);
}
EXPORT_SYMBOL_GPL(rust_helper_get_task_struct);

void rust_helper_put_task_struct(struct task_struct *t)
{
	put_task_struct(t);
}
EXPORT_SYMBOL_GPL(rust_helper_put_task_struct);

int rust_helper_security_binder_set_context_mgr(const struct cred *mgr)
{
	return security_binder_set_context_mgr(mgr);
}
EXPORT_SYMBOL_GPL(rust_helper_security_binder_set_context_mgr);

int rust_helper_security_binder_transaction(const struct cred *from,
					    const struct cred *to)
{
	return security_binder_transaction(from, to);
}
EXPORT_SYMBOL_GPL(rust_helper_security_binder_transaction);

int rust_helper_security_binder_transfer_binder(const struct cred *from,
						const struct cred *to)
{
	return security_binder_transfer_binder(from, to);
}
EXPORT_SYMBOL_GPL(rust_helper_security_binder_transfer_binder);

int rust_helper_security_binder_transfer_file(const struct cred *from,
					      const struct cred *to,
					      struct file *file)
{
	return security_binder_transfer_file(from, to, file);
}
EXPORT_SYMBOL_GPL(rust_helper_security_binder_transfer_file);

struct file *rust_helper_get_file(struct file *f)
{
	return get_file(f);
}
EXPORT_SYMBOL_GPL(rust_helper_get_file);

void rust_helper_rcu_read_lock(void)
{
	rcu_read_lock();
}
EXPORT_SYMBOL_GPL(rust_helper_rcu_read_lock);

void rust_helper_rcu_read_unlock(void)
{
	rcu_read_unlock();
}
EXPORT_SYMBOL_GPL(rust_helper_rcu_read_unlock);

void rust_helper_synchronize_rcu(void)
{
	synchronize_rcu();
}
EXPORT_SYMBOL_GPL(rust_helper_synchronize_rcu);

void *rust_helper_dev_get_drvdata(struct device *dev)
{
	return dev_get_drvdata(dev);
}
EXPORT_SYMBOL_GPL(rust_helper_dev_get_drvdata);

void rust_helper_dev_set_drvdata(struct device *dev, void *data)
{
	dev_set_drvdata(dev, data);
}
EXPORT_SYMBOL_GPL(rust_helper_dev_set_drvdata);

const char *rust_helper_dev_name(const struct device *dev)
{
	return dev_name(dev);
}
EXPORT_SYMBOL_GPL(rust_helper_dev_name);

void rust_helper___seqcount_init(seqcount_t *s, const char *name,
				 struct lock_class_key *key)
{
	__seqcount_init(s, name, key);
}
EXPORT_SYMBOL_GPL(rust_helper___seqcount_init);

unsigned rust_helper_read_seqcount_begin(seqcount_t *s)
{
	return read_seqcount_begin(s);
}
EXPORT_SYMBOL_GPL(rust_helper_read_seqcount_begin);

int rust_helper_read_seqcount_retry(seqcount_t *s, unsigned start)
{
	return read_seqcount_retry(s, start);
}
EXPORT_SYMBOL_GPL(rust_helper_read_seqcount_retry);

void rust_helper_write_seqcount_begin(seqcount_t *s)
{
	do_write_seqcount_begin(s);
}
EXPORT_SYMBOL_GPL(rust_helper_write_seqcount_begin);

void rust_helper_write_seqcount_end(seqcount_t *s)
{
	do_write_seqcount_end(s);
}
EXPORT_SYMBOL_GPL(rust_helper_write_seqcount_end);

void rust_helper_irq_set_handler_locked(struct irq_data *data,
					irq_flow_handler_t handler)
{
	irq_set_handler_locked(data, handler);
}
EXPORT_SYMBOL_GPL(rust_helper_irq_set_handler_locked);

void *rust_helper_irq_data_get_irq_chip_data(struct irq_data *d)
{
	return irq_data_get_irq_chip_data(d);
}
EXPORT_SYMBOL_GPL(rust_helper_irq_data_get_irq_chip_data);

struct irq_chip *rust_helper_irq_desc_get_chip(struct irq_desc *desc)
{
	return irq_desc_get_chip(desc);
}
EXPORT_SYMBOL_GPL(rust_helper_irq_desc_get_chip);

void *rust_helper_irq_desc_get_handler_data(struct irq_desc *desc)
{
	return irq_desc_get_handler_data(desc);
}
EXPORT_SYMBOL_GPL(rust_helper_irq_desc_get_handler_data);

void rust_helper_chained_irq_enter(struct irq_chip *chip,
				   struct irq_desc *desc)
{
	chained_irq_enter(chip, desc);
}
EXPORT_SYMBOL_GPL(rust_helper_chained_irq_enter);

void rust_helper_chained_irq_exit(struct irq_chip *chip,
				   struct irq_desc *desc)
{
	chained_irq_exit(chip, desc);
}
EXPORT_SYMBOL_GPL(rust_helper_chained_irq_exit);

const struct cred *rust_helper_get_cred(const struct cred *cred)
{
	return get_cred(cred);
}
EXPORT_SYMBOL_GPL(rust_helper_get_cred);

void rust_helper_put_cred(const struct cred *cred)
{
	put_cred(cred);
}
EXPORT_SYMBOL_GPL(rust_helper_put_cred);

const struct of_device_id *rust_helper_of_match_device(
		const struct of_device_id *matches, const struct device *dev)
{
	return of_match_device(matches, dev);
}
EXPORT_SYMBOL_GPL(rust_helper_of_match_device);

void rust_helper_init_completion(struct completion *c)
{
	init_completion(c);
}
EXPORT_SYMBOL_GPL(rust_helper_init_completion);

struct sk_buff *rust_helper_skb_get(struct sk_buff *skb)
{
	return skb_get(skb);
}
EXPORT_SYMBOL_GPL(rust_helper_skb_get);

unsigned int rust_helper_skb_headlen(const struct sk_buff *skb)
{
	return skb_headlen(skb);
}
EXPORT_SYMBOL_GPL(rust_helper_skb_headlen);

void rust_helper_dev_hold(struct net_device *dev)
{
	return dev_hold(dev);
}
EXPORT_SYMBOL_GPL(rust_helper_dev_hold);

void rust_helper_dev_put(struct net_device *dev)
{
	return dev_put(dev);
}
EXPORT_SYMBOL_GPL(rust_helper_dev_put);

struct net *rust_helper_get_net(struct net *net)
{
	return get_net(net);
}
EXPORT_SYMBOL_GPL(rust_helper_get_net);

void rust_helper_put_net(struct net *net)
{
	return put_net(net);
}
EXPORT_SYMBOL_GPL(rust_helper_put_net);

unsigned int rust_helper_NF_QUEUE_NR(unsigned int n)
{
	return NF_QUEUE_NR(n);
}
EXPORT_SYMBOL_GPL(rust_helper_NF_QUEUE_NR);

void rust_helper___INIT_WORK_WITH_KEY(struct work_struct *work,
		work_func_t func, bool on_stack, struct lock_class_key *key)
{
	__INIT_WORK_WITH_KEY(work, func, on_stack, key);
}
EXPORT_SYMBOL_GPL(rust_helper___INIT_WORK_WITH_KEY);

struct dentry *rust_helper_dget(struct dentry *dentry)
{
	return dget(dentry);
}
EXPORT_SYMBOL_GPL(rust_helper_dget);

void rust_helper_lockdep_register_key(struct lock_class_key *key)
{
	lockdep_register_key(key);
}
EXPORT_SYMBOL_GPL(rust_helper_lockdep_register_key);

void rust_helper_lockdep_unregister_key(struct lock_class_key *key)
{
	lockdep_unregister_key(key);
}
EXPORT_SYMBOL_GPL(rust_helper_lockdep_unregister_key);

int rust_helper_fs_parse(struct fs_context *fc,
		const struct fs_parameter_spec *desc,
		struct fs_parameter *param,
		struct fs_parse_result *result)
{
	return fs_parse(fc, desc, param, result);
}
EXPORT_SYMBOL_GPL(rust_helper_fs_parse);

void rust_helper_pci_set_drvdata(struct pci_dev *pdev, void *data)
{
    pci_set_drvdata(pdev, data);
}
EXPORT_SYMBOL_GPL(rust_helper_pci_set_drvdata);

void *rust_helper_pci_get_drvdata(struct pci_dev *pdev)
{
    return pci_get_drvdata(pdev);
}
EXPORT_SYMBOL_GPL(rust_helper_pci_get_drvdata);

dma_addr_t rust_helper_dma_map_single_attrs(struct device *dev, void *ptr,
					    size_t size, enum dma_data_direction dir,
					    unsigned long attrs)
{
	return dma_map_single_attrs(dev, ptr, size, dir, attrs);
}
EXPORT_SYMBOL_GPL(rust_helper_dma_map_single_attrs);

void rust_helper_dma_unmap_single_attrs(struct device *dev, dma_addr_t addr,
					size_t size, enum dma_data_direction dir,
					unsigned long attrs)
{
	dma_unmap_single_attrs(dev, addr, size, dir, attrs);
}
EXPORT_SYMBOL_GPL(rust_helper_dma_unmap_single_attrs);

int rust_helper_dma_mapping_error(struct device *dev, dma_addr_t dma_addr)
{
	return dma_mapping_error(dev, dma_addr);
}
EXPORT_SYMBOL_GPL(rust_helper_dma_mapping_error);

void *rust_helper_dma_alloc_coherent(struct device *dev, size_t size,
				     dma_addr_t *dma_handle, gfp_t gfp)
{
	return dma_alloc_coherent(dev, size, dma_handle, gfp);
}
EXPORT_SYMBOL_GPL(rust_helper_dma_alloc_coherent);

void rust_helper_dma_free_coherent(struct device *dev, size_t size,
				   void *cpu_addr, dma_addr_t dma_handle)
{
	return dma_free_coherent(dev, size, cpu_addr, dma_handle);
}
EXPORT_SYMBOL_GPL(rust_helper_dma_free_coherent);

#ifdef CONFIG_NET
void rust_helper_eth_hw_addr_set(struct net_device *dev, const u8 *addr)
{
	eth_hw_addr_set(dev, addr);
}
EXPORT_SYMBOL_GPL(rust_helper_eth_hw_addr_set);

void rust_helper_netif_start_queue(struct net_device *dev)
{
	netif_start_queue(dev);
}
EXPORT_SYMBOL_GPL(rust_helper_netif_start_queue);

void rust_helper_netif_stop_queue(struct net_device *dev) {
	netif_stop_queue(dev);
}
EXPORT_SYMBOL_GPL(rust_helper_netif_stop_queue);

void rust_helper_netdev_sent_queue(struct net_device *dev, unsigned int bytes) {
	return netdev_sent_queue(dev, bytes);
}
EXPORT_SYMBOL_GPL(rust_helper_netdev_sent_queue);

struct sk_buff *rust_helper_netdev_alloc_skb_ip_align(struct net_device *dev,
						      unsigned int length) {
	return netdev_alloc_skb_ip_align(dev, length);
}
EXPORT_SYMBOL_GPL(rust_helper_netdev_alloc_skb_ip_align);

int rust_helper_skb_put_padto(struct sk_buff *skb, unsigned int len) {
	return skb_put_padto(skb, len);
}
EXPORT_SYMBOL_GPL(rust_helper_skb_put_padto);

void rust_helper_netdev_completed_queue(struct net_device *dev, unsigned int pkts,
	unsigned int bytes) {
	netdev_completed_queue(dev, pkts, bytes);
}
EXPORT_SYMBOL_GPL(rust_helper_netdev_completed_queue);

#endif

u8 rust_helper_inb(unsigned long addr)
{
	return inb(addr);
}
EXPORT_SYMBOL_GPL(rust_helper_inb);

u16 rust_helper_inw(unsigned long addr)
{
	return inw(addr);
}
EXPORT_SYMBOL_GPL(rust_helper_inw);

u32 rust_helper_inl(unsigned long addr)
{
	return inl(addr);
}
EXPORT_SYMBOL_GPL(rust_helper_inl);

void rust_helper_outb(u8 value, unsigned long addr)
{
	outb(value, addr);
}
EXPORT_SYMBOL_GPL(rust_helper_outb);

void rust_helper_outw(u16 value, unsigned long addr)
{
	outw(value, addr);
}
EXPORT_SYMBOL_GPL(rust_helper_outw);

void rust_helper_outl(u32 value, unsigned long addr)
{
	outl(value, addr);
}
EXPORT_SYMBOL_GPL(rust_helper_outl);

void rust_helper_mdelay(unsigned long usecs) {
	mdelay(usecs);
}
EXPORT_SYMBOL_GPL(rust_helper_mdelay);

void rust_helper_udelay(unsigned long usecs) {
	udelay(usecs);
}
EXPORT_SYMBOL_GPL(rust_helper_udelay);

void rust_helper_ndelay(unsigned long nsecs) {
	ndelay(nsecs);
}
EXPORT_SYMBOL_GPL(rust_helper_ndelay);

int rust_helper_nla_put_u8(struct sk_buff *skb, int attrtype, u8 value)
{
	return nla_put_u8(skb , attrtype , value);
}
EXPORT_SYMBOL_GPL(rust_helper_nla_put_u8);

int rust_helper_nla_put_u32(struct sk_buff *skb, int attrtype, u32 value)
{
	return nla_put_u32(skb , attrtype , value);
}
EXPORT_SYMBOL_GPL(rust_helper_nla_put_u32);

int rust_helper_nla_put_string(struct sk_buff *skb, int attrtype, const char *str)
{
return nla_put_string(skb , attrtype , str);
}
EXPORT_SYMBOL_GPL(rust_helper_nla_put_string);



struct sk_buff *rust_helper_genlmsg_new(size_t payload, gfp_t flags)
{
	return genlmsg_new(payload,flags) ; 
}
EXPORT_SYMBOL_GPL(rust_helper_genlmsg_new);

void *rust_helper_genlmsg_put(struct sk_buff *skb, u32 portid, u32 seq,const struct genl_family *family, int flags, u8 cmd){

	return genlmsg_put(skb,portid,seq,family,flags,cmd) ; 
}
EXPORT_SYMBOL_GPL(rust_helper_genlmsg_put);

void rust_helper_genlmsg_end(struct sk_buff *skb, void *hdr) {

	return genlmsg_end(skb,hdr) ;  
}
EXPORT_SYMBOL_GPL(rust_helper_genlmsg_end);

int rust_helper_genlmsg_unicast(struct net *net, struct sk_buff *skb, u32 portid){

	return genlmsg_unicast(net,skb,portid) ; 
}
EXPORT_SYMBOL_GPL(rust_helper_genlmsg_unicast);

struct net *rust_helper_genl_info_net(struct genl_info *info) {

	return genl_info_net(info) ; 
}
EXPORT_SYMBOL_GPL(rust_helper_genl_info_net); 



int rust_helper_genl_register_family(struct genl_family *family){
	return genl_register_family(family) ; 
}
EXPORT_SYMBOL_GPL(rust_helper_genl_register_family);

int rust_helper_genl_unregister_family(struct genl_family *family){
	return genl_unregister_family(family) ; 
}
EXPORT_SYMBOL_GPL(rust_helper_genl_unregister_family);



/*************************************************/
struct sk_buff *rust_helper_nlmsg_new(size_t payload, gfp_t flags)
{
	return nlmsg_new(payload,flags) ; 
}
EXPORT_SYMBOL_GPL(rust_helper_nlmsg_new);

struct nlmsghdr *rust_helper_nlmsg_put(struct sk_buff *skb, u32 portid, u32 seq,int type, int payload, int flags){

	return nlmsg_put(skb,portid,seq,type,payload,flags) ; 
}
EXPORT_SYMBOL_GPL(rust_helper_nlmsg_put);


void rust_helper_nlmsg_end(struct sk_buff *skb, struct nlmsghdr *nlh) {

	return nlmsg_end(skb,nlh) ;  
}
EXPORT_SYMBOL_GPL(rust_helper_nlmsg_end);




int rust_helper_nlmsg_unicast(struct sock *sk, struct sk_buff *skb, u32 portid){

	return nlmsg_unicast(sk,skb,portid) ; 
}
EXPORT_SYMBOL_GPL(rust_helper_nlmsg_unicast);


void *rust_helper_nla_data(const struct nlattr *nla)
{
	return nla_data(nla) ; 
}
EXPORT_SYMBOL_GPL(rust_helper_nla_data);


/******************************************************/


void rust_helper_list_add_tail(struct list_head *new, struct list_head *head){
	return  list_add_tail(new,head) ; 
}
EXPORT_SYMBOL_GPL(rust_helper_list_add_tail);

/*****************************************************/

bool rust_helper_netif_device_present(const struct net_device *dev)
{
	return netif_device_present(dev);
}
EXPORT_SYMBOL_GPL(rust_helper_netif_device_present);

void rust_helper_netdev_priv(const struct net_device *dev)
{
	netdev_priv(dev);
}
EXPORT_SYMBOL_GPL(rust_helper_netdev_priv);

void rust_helper_hlist_add_head_rcu(struct hlist_node *n,struct hlist_head *h)
{
    return hlist_add_head_rcu(n,h) ; 
}
EXPORT_SYMBOL_GPL(rust_helper_hlist_add_head_rcu);

void rust_helper_hlist_del_rcu(struct hlist_node *n){
    return hlist_del_rcu(n); 
}
EXPORT_SYMBOL_GPL(rust_helper_hlist_del_rcu);

void rust_helper_skb_set_transport_header(struct sk_buff *skb,const int offset) {
	return skb_set_transport_header(skb,offset) ; 
}
EXPORT_SYMBOL_GPL(rust_helper_skb_set_transport_header);

void rust_helper_skb_set_network_header(struct sk_buff *skb , const int offset) {
	return skb_set_network_header(skb,offset) ; 
}
EXPORT_SYMBOL_GPL(rust_helper_skb_set_network_header);

void rust_helper_skb_set_mac_header(struct sk_buff *skb, const int offset) {
	return skb_set_mac_header(skb,offset) ; 
}
EXPORT_SYMBOL_GPL(rust_helper_skb_set_mac_header);

void rust_helper_skb_reserve (struct sk_buff *skb, int len ) {
	return skb_set_mac_header(skb,len) ; 
}
EXPORT_SYMBOL_GPL(rust_helper_skb_reserve);


unsigned char *rust_helper_skb_network_header(const struct sk_buff *skb) {
	return skb_network_header(skb) ; 
}
EXPORT_SYMBOL_GPL(rust_helper_skb_network_header);

unsigned char *rust_helper_skb_tail_pointer(const struct sk_buff *skb) {
	return skb_tail_pointer(skb) ; 
}
EXPORT_SYMBOL_GPL(rust_helper_skb_tail_pointer);

struct sk_buff *rust_helper_netdev_alloc_skb(struct net_device *dev,unsigned int length) {
	return netdev_alloc_skb(dev,length) ; 
}
EXPORT_SYMBOL_GPL(rust_helper_netdev_alloc_skb);


struct ethhdr *rust_helper_eth_hdr(const struct sk_buff *skb) {
	return eth_hdr(skb) ; 
}
EXPORT_SYMBOL_GPL(rust_helper_eth_hdr);

unsigned int rust_helper_ip_hdrlen(const struct sk_buff *skb) {
	return ip_hdrlen(skb) ;
}
EXPORT_SYMBOL_GPL(rust_helper_ip_hdrlen);

void rust_helper_hlist_add_head(struct hlist_node *n, struct hlist_head *h) {
	return hlist_add_head(n,h) ; 
}
EXPORT_SYMBOL_GPL(rust_helper_hlist_add_head);


/*
 * We use `bindgen`'s `--size_t-is-usize` option to bind the C `size_t` type
 * as the Rust `usize` type, so we can use it in contexts where Rust
 * expects a `usize` like slice (array) indices. `usize` is defined to be
 * the same as C's `uintptr_t` type (can hold any pointer) but not
 * necessarily the same as `size_t` (can hold the size of any single
 * object). Most modern platforms use the same concrete integer type for
 * both of them, but in case we find ourselves on a platform where
 * that's not true, fail early instead of risking ABI or
 * integer-overflow issues.
 *
 * If your platform fails this assertion, it means that you are in
 * danger of integer-overflow bugs (even if you attempt to remove
 * `--size_t-is-usize`). It may be easiest to change the kernel ABI on
 * your platform such that `size_t` matches `uintptr_t` (i.e., to increase
 * `size_t`, because `uintptr_t` has to be at least as big as `size_t`).
 */
static_assert(
	sizeof(size_t) == sizeof(uintptr_t) &&
	__alignof__(size_t) == __alignof__(uintptr_t),
	"Rust code expects C `size_t` to match Rust `usize`"
);
