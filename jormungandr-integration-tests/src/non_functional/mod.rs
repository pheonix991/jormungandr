/*
 Sanity performacne tests. Quick tests to check overall node performance.
 Run some transaction for ~15 minutes or specified no of transactions (100)
*/
pub mod sanity;
/*
Long running test for self node (48 h)
*/
pub mod soak;

use crate::common::{jcli_wrapper, jormungandr::JormungandrProcess};
use jormungandr_lib::testing::Thresholds;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeStuckError {
    #[error("node tip is not moving up. Stuck at {tip_hash} ")]
    TipIsNotMoving { tip_hash: String, logs: String },
    #[error("node block counter is not moving up. Stuck at {block_counter}")]
    BlockCounterIsNoIncreased { block_counter: u64, logs: String },
}

pub fn thresholds_for_transaction_counter(counter: u64) -> Thresholds<u64> {
    let green = (counter / 2) as u64;
    let yellow = (counter / 3) as u64;
    let red = (counter / 4) as u64;
    Thresholds::<u64>::new(green, yellow, red, counter)
}

pub fn thresholds_for_transaction_send_by_duration(duration: Duration) -> Thresholds<Duration> {
    let green = Duration::from_secs(duration.as_secs() / 2);
    let yellow = Duration::from_secs(duration.as_secs() / 3);
    let red = Duration::from_secs(duration.as_secs() / 4);
    Thresholds::<Duration>::new(green, yellow, red, duration)
}

pub fn send_transaction_and_ensure_block_was_produced(
    transation_messages: &Vec<String>,
    jormungandr: &JormungandrProcess,
) -> Result<(), NodeStuckError> {
    let block_tip_before_transaction =
        jcli_wrapper::assert_rest_get_block_tip(&jormungandr.rest_address());
    let block_counter_before_transaction = jormungandr.logger.get_created_blocks_counter();

    jcli_wrapper::assert_all_transactions_in_block(&transation_messages, &jormungandr);

    let block_tip_after_transaction =
        jcli_wrapper::assert_rest_get_block_tip(&jormungandr.rest_address());
    let block_counter_after_transaction = jormungandr.logger.get_created_blocks_counter();

    if block_tip_before_transaction == block_tip_after_transaction {
        return Err(NodeStuckError::TipIsNotMoving {
            tip_hash: block_tip_after_transaction.clone(),
            logs: jormungandr.logger.get_log_content(),
        });
    }

    if block_counter_before_transaction == block_counter_after_transaction {
        return Err(NodeStuckError::BlockCounterIsNoIncreased {
            block_counter: block_counter_before_transaction as u64,
            logs: jormungandr.logger.get_log_content(),
        });
    }

    Ok(())
}
