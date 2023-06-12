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
				"Memory Usage: {}\nCPU Usage: {}%",
				format_bytes(process.memory()),
				process.cpu_usage().round(),
			);

			ctx.send(|m| m.content(process_info)).await?;
		}
	}

	Ok(())
}

fn format_bytes(bytes: u64) -> String {
	if bytes == 0 {
		return String::from("0 bytes");
	}

	let k: u64 = 1024;
	let sizes = ["bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
	let mut i = 0;
	let mut num = bytes as f64;

	while num >= k as f64 && i < sizes.len() - 1 {
		num /= k as f64;
		i += 1;
	}

	format!("{} {}", num.round(), sizes[i])
}
