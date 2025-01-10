use crate::core::*;

///
/// A buffer for transferring a set of uniform variables to the shader program
/// (see also [use_uniform_block](crate::core::Program::use_uniform_block)).
///
pub struct AtomicCounterBuffer {
    context: Context,
    id: crate::context::Buffer,
    length: usize,
}

impl AtomicCounterBuffer {
    ///
    /// Creates a new atomic counter buffer.
    /// The variables are initialized to 0.
    ///
    pub fn new(context: &Context, length: usize) -> AtomicCounterBuffer {
        let id = unsafe { context.create_buffer().expect("Failed creating buffer") };

        let mut buffer = AtomicCounterBuffer {
            context: context.clone(),
            id,
            length,
        };
        buffer.fill(&vec![0; length]);
        buffer
    }

    pub(crate) fn fill(&mut self, data: &[u32]) {
        if data.len() != self.length {
            panic!("Invalid data length");
        }
        self.bind();
        unsafe {
            self.context.buffer_data_u8_slice(
                crate::context::ATOMIC_COUNTER_BUFFER,
                to_byte_slice(data),
                crate::context::DYNAMIC_COPY,
            );
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    /// Reads the atomic counter buffer
    pub fn read(&self, data: &mut [u32]) {
        if data.len() != self.length {
            panic!("Invalid data length");
        }
        self.memory_barrier();
        unsafe {
            self.context.get_buffer_sub_data(
                crate::context::ATOMIC_COUNTER_BUFFER,
                0,
                to_byte_slice_mut(data),
            );
        }
    }

    pub(crate) fn bind(&self) {
        unsafe {
            self.context
                .bind_buffer(crate::context::ATOMIC_COUNTER_BUFFER, Some(self.id))
        };
    }

    pub(crate) fn bind_base(&self, id: u32) {
        unsafe {
            self.context
                .bind_buffer_base(crate::context::ATOMIC_COUNTER_BUFFER, id, Some(self.id))
        };
    }

    pub(crate) fn memory_barrier(&self) {
        self.bind();
        unsafe {
            self.context
                .memory_barrier(crate::context::ATOMIC_COUNTER_BARRIER_BIT);
        }
    }
}

impl Drop for AtomicCounterBuffer {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_buffer(self.id);
        }
    }
}
