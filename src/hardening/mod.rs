use winreg::enums::*;
use winreg::RegKey;
use std::io;

pub fn apply_vaccine() -> io::Result<()> {
    // 1. Accedemos a HKEY_LOCAL_MACHINE (Requiere Admin)
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    
    // 2. Definimos la ruta de la política de IA
    // Si la ruta no existe, create_subkey la crea automáticamente.
    let (key, _disp) = hklm.create_subkey("SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsAI")?;
    
    // 3. Inyectamos el veneno para la IA (DisableAIDataAnalysis = 1)
    key.set_value("DisableAIDataAnalysis", &1u32)?;
    
    Ok(())
}

pub fn remove_vaccine() -> io::Result<()> {
    // Para el comando --restore (siempre dejamos una salida)
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    
    // Intentamos abrir la llave con permisos de escritura
    if let Ok(key) = hklm.open_subkey_with_flags("SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsAI", KEY_WRITE) {
        // Borramos el valor
        key.delete_value("DisableAIDataAnalysis")?;
    }
    
    Ok(())
}