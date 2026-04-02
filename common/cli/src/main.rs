use common_adapters::AdapterRegistry;
use common_cli::{execute, parse_cli_args};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let invocation = match parse_cli_args(&args) {
        Ok(invocation) => invocation,
        Err(error) => {
            eprintln!("error: {error}");
            eprintln!("usage:");
            eprintln!("  common dev --surface <web|desktop|docs|demo> [--host HOST] [--port PORT] [--route PATH]");
            eprintln!("  common demo");
            eprintln!("  common docs");
            eprintln!("  common release");
            eprintln!("  common review --list-hosts [--config PATH]");
            eprintln!("  common review --ssh-host ALIAS [--config PATH] [--path REMOTE_PATH]...");
            std::process::exit(2);
        }
    };

    match execute(&invocation, &AdapterRegistry::default()) {
        Ok(bundle) => {
            for message in bundle.messages {
                println!("{message}");
            }
            for plan in bundle.plans {
                println!("[{}] {:?}", plan.adapter_id, plan.plan_kind);
                println!("{}", plan.summary);
            }
        }
        Err(error) => {
            eprintln!("dispatch error: {error}");
            std::process::exit(1);
        }
    }
}
