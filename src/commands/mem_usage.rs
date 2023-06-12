use crate::{FloopyContext, FloopyError};
use sysinfo::{ProcessExt, System, SystemExt};

/// Displays the bot's memory usage.
#[poise::command(
	slash_command,
	prefix_command,
	rename = "mem",
	aliases("mu", "memusage", "mem_usage", "memory_usage")
)]
pub async fn command(ctx: FloopyContext<'_>) -> Result<(), FloopyError> {
	let mut sys = System::new_all();

	sys.refresh_memory();

	if let Ok(pid) = sysinfo::get_current_pid() {
		if let Some(process) = sys.process(pid) {
			let process_info = format!(
				"Memory Usage: {} MB\nCPU Usage: {}%",
				process.memory() / 1000000,
				process.cpu_usage().round(),
			);

			ctx.send(|m| m.content(process_info)).await?;
		}
	}

	Ok(())
}
