mod detector;
mod hardening;
mod persistence;

use clap::{Parser, Subcommand};
use colored::*;
use detector::{SystemState, WindowsVariant};
use is_elevated::is_elevated;
use std::process;

#[derive(Parser)]
#[command(name = "Anestesia")]
#[command(author = "Senior DevSecOps")]
#[command(version = "0.2.0")]
#[command(about = "Herramienta de soberanía digital para bloquear Windows Recall e IA.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Status,
    Lock,
    Restore,
    // Instala la persistencia (Se auto-ejecuta al inicio)
    Sentinel {
        #[arg(short, long)]
        remove: bool, // Flag para desinstalar: --remove
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Status => run_status(),
        Commands::Lock => run_lock(),
        Commands::Restore => run_restore(),
        Commands::Sentinel { remove } => run_sentinel(*remove),
    }
}

fn run_status() {
    println!("{}", "--- ANESTESIA: REPORTE DE ESTADO ---".cyan().bold());
    let state = SystemState::scan();

    match state.os_variant {
        WindowsVariant::Windows10 => println!("Sistema: {}", "Windows 10".blue()),
        WindowsVariant::Windows11Standard => println!("Sistema: {}", "Windows 11".magenta()),
        WindowsVariant::Windows11Copilot => println!("Sistema: {}", "Windows 11 Copilot+".red().bold()),
        _ => println!("Sistema: Desconocido"),
    }

    if state.policy_disabled {
        println!("Vacuna GPO: {}", "ACTIVA (Protegido)".green().bold());
    } else {
        println!("Vacuna GPO: {}", "INACTIVA (Vulnerable)".yellow());
    }
    
    if state.recall_folder_exists {
         println!("Almacenamiento Recall: {}", "DETECTADO".red().bold());
    } else {
         println!("Almacenamiento Recall: {}", "LIMPIO".green());
    }
}

fn run_lock() {
    check_admin();
    // Modo silencioso para el Sentinel
    match hardening::apply_vaccine() {
        Ok(_) => println!("{}", "✔ Vacuna de Registro inyectada.".green().bold()),
        Err(e) => println!("{} Fallo al inyectar vacuna: {}", "✖".red(), e),
    }
}

fn run_restore() {
    check_admin();
    println!("{}", "--- REVIRTIENDO CAMBIOS ---".blue());
    match hardening::remove_vaccine() {
        Ok(_) => println!("{}", "✔ Vacuna eliminada.".green()),
        Err(e) => println!("{} Fallo al remover vacuna: {}", "✖".red(), e),
    }
}

fn run_sentinel(remove: bool) {
    check_admin();
    
    if remove {
        println!("{}", "Desactivando vigilancia...".yellow());
        match persistence::uninstall_sentinel() {
            Ok(_) => println!("{}", "✔ Centinela eliminado. Windows Update podría reactivar Recall.".green()),
            Err(e) => println!("{} Error al borrar tarea: {}", "✖".red(), e),
        }
    } else {
        println!("{}", "Instalando Centinela (Persistencia)...".magenta());
        match persistence::install_sentinel() {
            Ok(_) => {
                println!("{}", "✔ Tarea 'AmnesiaSentinel' creada.".green().bold());
                println!("   > Trigger: Al iniciar sesión (ONLOGON)");
                println!("   > Acción:  Ejecutar 'anestesia lock'");
                println!("   > Nivel:   Privilegios más altos");
            },
            Err(e) => println!("{} Error al crear tarea: {}", "✖".red(), e),
        }
    }
}

fn check_admin() {
    if !is_elevated() {
        eprintln!("{}", "Error: Se requieren permisos de Administrador.".red().bold());
        process::exit(1);
    }
}