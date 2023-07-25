use concordium_rust_sdk::types::smart_contracts::concordium_contracts_common::{
    Address, ModuleReference, OwnedEntrypointName, OwnedParameter, Serial, Write,
};

#[derive(Debug)]
pub struct CIS2BridgeableInitParams {
    pub url:  String,
    pub hash: Option<[u8; 32]>,
}

/// Serialization for the withdraw contract function parameter.
impl Serial for CIS2BridgeableInitParams {
    fn serial<W: Write>(&self, out: &mut W) -> Result<(), W::Err> {
        self.url.serial(out)?;
        self.hash.serial(out)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct BridgeGrantRoleParams {
    pub address: Address,
    pub role:    BridgeRoles,
}

impl Serial for BridgeGrantRoleParams {
    fn serial<W: Write>(&self, out: &mut W) -> Result<(), W::Err> {
        self.address.serial(out)?;
        self.role.serial(out)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum BridgeRoles {
    Admin,
    Mapper,
    StateSyncer,
}

impl Serial for BridgeRoles {
    fn serial<W: Write>(&self, out: &mut W) -> Result<(), W::Err> {
        match self {
            BridgeRoles::Admin => out.write_u8(0),
            BridgeRoles::Mapper => out.write_u8(1),
            BridgeRoles::StateSyncer => out.write_u8(2),
        }
    }
}

#[derive(Debug)]
pub struct CIS2BridgeableGrantRoleParams {
    pub address: Address,
    pub role:    CIS2BridgeableRoles,
}

impl Serial for CIS2BridgeableGrantRoleParams {
    fn serial<W: Write>(&self, out: &mut W) -> Result<(), W::Err> {
        self.address.serial(out)?;
        self.role.serial(out)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum CIS2BridgeableRoles {
    Admin,
    Manager,
}

impl Serial for CIS2BridgeableRoles {
    fn serial<W: Write>(&self, out: &mut W) -> Result<(), W::Err> {
        match self {
            CIS2BridgeableRoles::Admin => out.write_u8(0),
            CIS2BridgeableRoles::Manager => out.write_u8(1),
        }
    }
}

#[derive(Debug)]
pub struct UpgradeParams {
    /// The new module reference.
    pub module:  ModuleReference,
    /// Optional entrypoint to call in the new module after upgrade.
    pub migrate: Option<(OwnedEntrypointName, OwnedParameter)>,
}

impl Serial for UpgradeParams {
    fn serial<W: Write>(&self, out: &mut W) -> Result<(), W::Err> {
        self.module.serial(out)?;
        self.migrate.serial(out)?;
        Ok(())
    }
}
