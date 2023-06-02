mod create;
mod update;

use hedera::AccountBalanceQuery;

use crate::common::{
    setup_global,
    TestEnvironment,
};

#[tokio::test]
async fn account_balance_query() -> anyhow::Result<()> {
    let TestEnvironment { config, client } = setup_global();

    let Some(op) = &config.operator else {
        log::debug!("skipping test due to lack of operator");

        return Ok(())
    };

    let balance = AccountBalanceQuery::new().account_id(op.account_id).execute(&client).await?;

    log::trace!("successfully queried balance: {balance:?}");

    anyhow::ensure!(balance.account_id == op.account_id);

    Ok(())
}
