use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;
use os_info::{Type, Version};
use serde::Serialize;

// Tipos tipos de entorno posibles
#[derive(Debug, Serialize, PartialEq)]
pub enum WindowsVariant {
    Windows10,          // Objetivo: Vacunación preventiva
    Windows11Standard,  // Objetivo: Hardening ligero
    Windows11Copilot,   // Objetivo: Protocolo Amnesia total (Recall activo)
    Unknown,
}

// Esta estructura es el reporte final para el usuario
#[derive(Debug, Serialize)]
pub struct SystemState {
    pub os_variant: WindowsVariant,
    pub recall_folder_exists: bool,
    pub policy_disabled: bool,
    pub recall_service_found: bool,
}

impl SystemState {
    // El método que escanea todo
    pub fn scan() -> Self {
        let os_variant = detect_os_variant();
        let recall_folder_exists = check_recall_folder();
        let policy_disabled = check_registry_policy();
        let recall_service_found = check_service_exists("ukgsvc"); // Servicio User Kernel Group

        // Refine la variante si encuentra evidencia física de Recall
        let final_variant = if recall_folder_exists {
            WindowsVariant::Windows11Copilot
        } else {
            os_variant
        };

        SystemState {
            os_variant: final_variant,
            recall_folder_exists,
            policy_disabled,
            recall_service_found,
        }
    }
}

fn detect_os_variant() -> WindowsVariant {
    let info = os_info::get();
    match info.os_type() {
        Type::Windows => {
            let version = info.version();
            // Lógica simple: Build 22000+ es Windows 11
            // Esto se puede mejorar, pero para MVP basta (por ahora).
            if let Version::Semantic(major, _, _) = version {
                if *major == 10 { return WindowsVariant::Windows10; }
            }
            // Asumimos Win11 por defecto si es Windows moderno
            WindowsVariant::Windows11Standard
        },
        _ => WindowsVariant::Unknown,
    }
}

fn check_recall_folder() -> bool {
    // Buscamos la carpeta en LocalAppData
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        let path_str = format!("{}\\CoreAIPlatform.00", local_app_data);
        let path = Path::new(&path_str);
        path.exists()
    } else {
        false
    }
}

fn check_registry_policy() -> bool {
    // Verifica si la "Vacuna" ya está aplicada en HKLM
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = "SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsAI";
    
    if let Ok(key) = hklm.open_subkey(path) {
        let val: Result<u32, _> = key.get_value("DisableAIDataAnalysis");
        return match val {
            Ok(1) => true,
            _ => false,
        };
    }
    false
}

// Chequeo de servicio (sin dependencias pesadas de API de Windows por ahora)
fn check_service_exists(_service_name: &str) -> bool {
    // TODO: Implementar check real de servicios en Fase 2 (Enforcer)
    false 
}