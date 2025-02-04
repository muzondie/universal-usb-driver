use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use windows::Win32::System::Com;
use super::detection::HardwareID;

#[derive(Debug, thiserror::Error)]
pub enum InstallError {
    #[error("Driver package corrupted")]
    InvalidPackage,
    #[error("System access denied")]
    PermissionDenied,
}

pub async fn install_driver(package: &[u8], hw_id: &HardwareID) -> Result<(), InstallError> {
    let temp_dir = std::env::temp_dir().join("uusbdrv");
    fs::create_dir_all(&temp_dir).await.map_err(|_| InstallError::PermissionDenied)?;
    
    let mut dest = File::create(temp_dir.join("driver.zip"))
        .await
        .map_err(|_| InstallError::PermissionDenied)?;

    dest.write_all(package)
        .await
        .map_err(|_| InstallError::InvalidPackage)?;

    unsafe {
        Com::CoInitializeEx(std::ptr::null_mut(), Com::COINIT_MULTITHREADED).ok()?;
        
        let mut pnp_util: windows::Win32::System::DeploymentServices::IPnpUtil = 
            Com::CoCreateInstance(
                &windows::Win32::System::DeploymentServices::PnpUtil,
                None,
                Com::CLSCTX_ALL,
            ).map_err(|_| InstallError::PermissionDenied)?;
            
        pnp_util.InstallDriverPackage(
            temp_dir.to_str().unwrap(),
            std::ptr::null_mut(),
            false,
            std::ptr::null_mut(),
        ).map_err(|_| InstallError::PermissionDenied)
    }
}