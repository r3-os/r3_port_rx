The Renesas RX port for [the R3 original kernel](::r3_kernel).

# Interrupts

TODO

# Kernel Timing

TODO

# Safety

Being a low-level piece of software, this port directly interfaces with hardware. This is not a problem as long as the port is the only piece of code doing that, but it might interfere with other low-level libraries and break their assumptions, potentially leading to an undefined behavior. This section lists potential harmful interactions that an application developer should keep in mind.

As a general thumb rule, you should not directly access hardware registers (e.g., `IPL` and `INTB`) and peripherals (e.g., the interrupt controller) that the port uses or exposes a standardized interface to access. You should access them only though the operating system.

## Stack Overflow

This port doesn't support detecting stack overflow.

# Implementation

## Processor states

*Case 1:* `CPU_LOCK_PRIORITY_MASK == 15 && SUPPORT_NESTING == false`

The hardware exception entry sequence clears `PSW.I`. This case exploits this property. CPU Lock is implemented by `PSW.IPL`.

|   Context   | CPU Lock | `PSW.I` | `PSW.IPL` | `PSW.U` |
| ----------- | -------- | ------- | --------- | ------- |
| Boot        | Active   | `0`     | `15`      | `?`     |
| Task        | Inactive | `1`     | `0`       | `1`     |
| Task        | Active   | `1`     | `15`      | `1`     |
| Interrupt   | Inactive | `0`     | `0`       | `0`     |
| Interrupt   | Active   | `0`     | `15`      | `0`     |
| Dispatcher¹ | Active   |         | `15`      | `1`     |

*Case 2:* `CPU_LOCK_PRIORITY_MASK != 15 && SUPPORT_NESTING == false`

`PSW.I` is always set to allow high-priority interrupts to be taken. The dispatcher can't borrow the interrupt stack by copying ISP to USP because a high-priority interrupt handler could write to memory locations beneath ISP.

|   Context   | CPU Lock | `PSW.I` |        `PSW.IPL`         |  `PSW.U`   |
| ----------- | -------- | ------- | ------------------------ | ---------- |
| Boot        | Active   | `1`     | `CPU_LOCK_PRIORITY_MASK` | See Case 1 |
| Task        | Inactive | `1`     | `0`                      | See Case 1 |
| Task        | Active   | `1`     | `CPU_LOCK_PRIORITY_MASK` | See Case 1 |
| Interrupt   | Inactive | `1`     | `CPU_LOCK_PRIORITY_MASK` | See Case 1 |
| Interrupt   | Active   | `1`     | `CPU_LOCK_PRIORITY_MASK` | See Case 1 |
| Dispatcher¹ | Active   |         | `CPU_LOCK_PRIORITY_MASK` | `0` or `1` |

*Case 3:* `CPU_LOCK_PRIORITY_MASK == 15 && SUPPORT_NESTING == true`

CPU Lock is implemented by `PSW.I`.

|   Context   | CPU Lock | `PSW.I` |      `PSW.IPL`       |  `PSW.U`   |
| ----------- | -------- | ------- | -------------------- | ---------- |
| Boot        | Active   | `0`     | `0`                  | See Case 1 |
| Task        | Inactive | `1`     | `0`                  | See Case 1 |
| Task        | Active   | `0`     | `0`                  | See Case 1 |
| Interrupt   | Inactive | `1`     | `interrupt_priority` | See Case 1 |
| Interrupt   | Active   | `0`     | `interrupt_priority` | See Case 1 |
| Dispatcher¹ | Active   | `0`     | `0`                  | See Case 1 |

*Case 4:* `CPU_LOCK_PRIORITY_MASK != 15 && SUPPORT_NESTING == true`

`PSW.I` is always set to allow high-priority interrupts to be taken. The dispatcher can't borrow the interrupt stack by copying ISP to USP because a high-priority interrupt handler could write to memory locations beneath ISP. Also note that `interrupt_priority` may be equal to `CPU_LOCK_PRIORITY_MASK`.

|   Context   | CPU Lock | `PSW.I` |        `PSW.IPL`         |  `PSW.U`   |
| ----------- | -------- | ------- | ------------------------ | ---------- |
| Boot        | Active   | `1`     | `CPU_LOCK_PRIORITY_MASK` | See Case 1 |
| Task        | Inactive | `1`     | `0`                      | See Case 1 |
| Task        | Active   | `1`     | `CPU_LOCK_PRIORITY_MASK` | See Case 1 |
| Interrupt   | Inactive | `1`     | `interrupt_priority`     | See Case 1 |
| Interrupt   | Active   | `1`     | `CPU_LOCK_PRIORITY_MASK` | See Case 1 |
| Dispatcher¹ | Active   |         | `CPU_LOCK_PRIORITY_MASK` | `0` or `1` |

¹ A **dispatcher context** is internal use only.

## Context state

The state of an interrupted thread is stored to the interrupted thread's stack in the following form:

```rust,ignore
#[repr(C)]
struct ContextState {
    // Second-level state (SLS)
    //
    // Includes everything that is not included in the first-level state. These
    // are moved between memory and registers only when switching tasks.
    // TODO: RXv2 has more `ACC`
    acc: [[u32; 2]; 1],
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    r12: u32,
    r13: u32,
    // TODO: `dr: [f64; 16]`, DPSW, etc. (RXv3)

    // First-level state (FLS)
    //
    // The GPR potion is comprised of caller-saved registers. In an exception
    // handler, saving/restoring this set of registers at entry and exit allows
    // it to call Rust functions.
    //
    // `{pc, psw}` is the sequence of registers that the RTE (return from
    // exception) instruction expects to be in memory in this exact order.
    fpsw: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r4: u32,
    r5: u32,
    r14: u32,
    r15: u32,
    pc: u32,	// stored only when switched away
    psw: u32,	// stored only when switched away

    // r0 (stack pointer) points to here
}
```

`r0` (stack pointer) is stored in [`TaskCb::port_task_state`].

[`TaskCb::port_task_state`]: r3_kernel::TaskCb::port_task_state

When a task is activated, a new context state is created inside the task's stack. By default, only essential registers are preloaded with known values. The **`preload-registers`** Cargo feature enables preloading for all GPRs, which might help in debugging at the cost of performance and code size.

For the idle task, saving and restoring the context store is essentially replaced with no-op or loads of hard-coded values. In particular, `pc` is always “restored” with the entry point of the idle task.


## Idle Task

When there is no task to schedule, the port transfers the control to **the idle task** (this is an internal construct and invisible to the kernel or an application). The idle task executes the `wait` instruction to reduce power consumption. This behavior can be changed by setting [`ThreadingOptions::USE_WAIT`][].

[`ThreadingOptions::USE_WAIT`]: crate::ThreadingOptions::USE_WAIT

The idle task always has `0` in `r0`.

## Register Preloading

When a task is activated, a new context state is created inside the task's stack. By default, only essential registers are preloaded with known values. The **`preload-registers`** Cargo feature enables preloading for all integer registers, which might help in debugging at the cost of performance and code size.
