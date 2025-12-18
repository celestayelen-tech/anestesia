Anestesia: Windows Sovereignty Tool

> Local Policy Enforcement Agent for Windows Recall & AI Telemetry.

Anestesia es una herramienta de sistemas diseñada para administradores y usuarios que desean mantener un control estricto sobre las características de IA de Microsoft (Windows Recall) en sus equipos. 

Funciona bajo el principio de Soberanía del Usuario: Tu máquina debe obedecer tus políticas de privacidad, incluso tras actualizaciones del sistema.

El sistema operativo registra actividad: qué apps abre, errores, estados del sistema, a veces incluso capturas o eventos visuales. Pero cualquier cosa puede filtrarse, romperse o ser atacada en el futuro.

No es que hoy sea peligroso, es que mañana puede serlo. Windows prioriza sus propias decisiones por sobre las del usuario, con funciones que miran y cambian cosas del sistema todo el tiempo, incluso después de desactivarlas. Yo priorizo al usuario y mi herramienta se asegura de que esas decisiones se respeten.

¿Es peligroso? No. Solo automatiza algo que normalmente hay que hacer a mano una y otra vez.

Anestesia no acusa a Windows de ser malicioso. Reduce la cantidad de información y estados que quedan persistidos sin necesidad.

¿Exagerado? Toda la ciberseugirdad existe porque alguien alguna vez dijo: "eso no es un problema"... Hasta que lo fue.

---

# Sobre del Proyecto Anestesia

1. Policy Agent, no Malware: Anestesia no utiliza técnicas de evasión, inyección de código ni hooks al Kernel. Utiliza las propias APIs de administración de Windows (GPO, Task Scheduler, Registry) para imponer un estado deseado.
2. Cero Telemetría: Esta herramienta es Air-Gapped por diseño. No inicia conexiones de red, no envía estadísticas y no busca actualizaciones automáticas. Lo que pasa en tu PC, se queda en tu PC.
3. Idempotencia: Las operaciones son seguras de repetir. Ejecutar el bloqueo múltiples veces no corrompe el sistema.
4. Fail-Open: Diseñada para no romper el arranque del sistema. Si la herramienta falla o Microsoft cambia la arquitectura, Anestesia alerta pero no interfiere con el funcionamiento crítico del OS.

---

# Detalles Técnicos (Transparencia)

Para garantizar la auditabilidad, documenté exactamente qué modifica Anestesia en tu sistema:

1. La Vacuna (GPO Enforcer)
Se establece la política de grupo local para deshabilitar el análisis de datos de IA.
* Target: `HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsAI`
* Value: `DisableAIDataAnalysis = 1` (DWORD)

2. El Centinela (Persistencia)
Para combatir la reversión automática tras Windows Updates, se instala una tarea programada ligera.
* Task Name: `AmnesiaSentinel`
* Trigger: `ONLOGON` (Al iniciar sesión del usuario).
* Action: Ejecuta `anestesia lock` para reaplicar la política si fue borrada.
* Resource Usage: ~0% CPU. El proceso se inicia, verifica el registro y termina en milisegundos.

---

# Uso (CLI)

Ejecuta la herramienta desde una terminal Powershell con privilegios de Administrador.

# Consultar Estado
Verifica si tu sistema es vulnerable o si Recall está activo.
```powershell
.\anestesia.exe status

Bloquear (Lock)
Aplica la vacuna en el Registro de Windows inmediatamente.

PowerShell

.\anestesia.exe lock

Instalar Centinela (Recomendado)
Instala la tarea programada para asegurar que el bloqueo persista tras reinicios y actualizaciones.

PowerShell

.\anestesia.exe sentinel

Para desinstalar el centinela: .\anestesia.exe sentinel --remove

Restaurar (Kill Switch)

Revierte TODOS los cambios. Elimina la clave del registro y devuelve el sistema a su configuración por defecto.

PowerShell

.\anestesia.exe restore

# Scope & Disclaimer

Compatibilidad: Probado en Windows 10 (21H2+) y Windows 11 (22H2, 23H2, 24H2).

Secure Boot: No requiere deshabilitar Secure Boot.

Limitaciones: Microsoft puede alterar la arquitectura de Recall en el futuro. Anestesia ofrece una mitigación basada en las políticas documentadas actuales ("Best Effort").

Descargo de Responsabilidad: Este software se proporciona "tal cual", sin garantía de ningún tipo, expresa o implícita. Anestesia está diseñado para auditorías de seguridad defensiva, diagnóstico y fines educativos, y solo debe utilizarse en sistemas y redes de su propiedad o que tenga autorización explícita para probar. La autora no se hace responsable de las consecuencias derivadas del mal uso de este software por uso indebido o no autorizado. Úselo bajo su propia responsabilidad y criterio técnico.

# Build & Integrity

Para compilar desde el código fuente (requiere Rust instalado):

Abra terminal:

cargo build --release

Verificación de Integridad: Siempre verifique el hash SHA-256 del binario descargado antes de ejecutarlo.

PowerShell

Get-FileHash .\anestesia.exe -Algorithm SHA256