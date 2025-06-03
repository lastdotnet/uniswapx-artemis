use std::error::Error;

use alloy::{dyn_abi::SolType, primitives::{Uint, I256, U256}, sol};
use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::sol_math::MulDiv;

fn current_timestamp_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("System time should never be before Unix epoch")
        .as_millis() as u64
}

sol! {
    #[derive(Debug, Deserialize)]
    struct OrderInfo {
        address reactor;
        address swapper;
        uint256 nonce;
        uint256 deadline;
        address additionalValidationContract;
        bytes additionalValidationData;
    }

    #[derive(Debug, Deserialize)]
    struct DutchOutput {
        address token;
        uint256 startAmount;
        uint256 endAmount;
        address recipient;
    }

    #[derive(Debug, Deserialize)]
    struct DutchInput {
        address token;
        uint256 startAmount;
        uint256 endAmount;
    }

    #[derive(Debug, Deserialize)]
    struct CosignerData {
        uint256 decayStartTime;
        uint256 decayEndTime;
        address exclusiveFiller;
        uint256 exclusivityOverrideBps;
        uint256 inputAmount;
        uint256[] outputAmounts;
    }

    #[derive(Debug, Deserialize)]
    struct V2DutchOrder {
        OrderInfo info;
        address cosigner;
        DutchInput baseInput;
        DutchOutput[] baseOutputs;
        CosignerData cosignerData;
        bytes cosignature;
    }

    #[derive(Debug, Deserialize)]
    struct PriorityInput {
        address token;
        uint256 amount;
        uint256 mpsPerPriorityFeeWei;
    }

    #[derive(Debug, Deserialize)]
    struct PriorityOutput {
        address token;
        uint256 amount;
        uint256 mpsPerPriorityFeeWei;
        address recipient;
    }

    #[derive(Debug, Deserialize)]
    struct PriorityCosignerData {
        uint256 auctionTargetBlock;
    }

    #[derive(Debug, Deserialize)]
    struct PriorityOrder {
        OrderInfo info;
        address cosigner;
        uint256 auctionStartBlock;
        uint256 baselinePriorityFeeWei;
        PriorityInput input;
        PriorityOutput[] outputs;
        PriorityCosignerData cosignerData;
        bytes cosignature;
    }

    #[derive(Debug, Deserialize)]
    struct V3DutchOrder {
        OrderInfo info;
        address cosigner;
        uint256 startingBaseFee;
        V3DutchInput baseInput;
        V3DutchOutput[] baseOutputs;
        V3CosignerData cosignerData;
        bytes cosignature;
    }

    #[derive(Debug, Deserialize)]
    struct V3CosignerData {
        uint256 decayStartBlock;
        address exclusiveFiller;
        uint256 exclusivityOverrideBps;
        uint256 inputAmount;
        uint256[] outputAmounts;
    }

    #[derive(Debug, Deserialize)]
    struct NonlinearDutchDecay {
        uint256 relativeBlocks;
        int256[] relativeAmounts;
    }

    #[derive(Debug, Deserialize)]
    struct V3DutchInput {
        address token;
        uint256 startAmount;
        NonlinearDutchDecay curve;
        uint256 maxAmount;
        uint256 adjustmentPerGweiBaseFee;
    }
    
    #[derive(Debug, Deserialize)]
    struct V3DutchOutput {
        address token;
        uint256 startAmount;
        NonlinearDutchDecay curve;
        address recipient;
        uint256 minAmount;
        uint256 adjustmentPerGweiBaseFee;
    }
}

pub const MPS: u64 = 1e7 as u64;
pub const BPS: U256 = Uint::from_limbs([10000, 0, 0, 0]);
const PACKED_UINT16_ARRAY_LENGTH: usize = 256 / 16;

#[derive(Debug, Clone, Deserialize)]
pub enum Order {
    V2DutchOrder(V2DutchOrder),
    PriorityOrder(PriorityOrder),
    V3DutchOrder(V3DutchOrder),
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TradeType {
    #[serde(rename = "exactIn")]
    ExactIn,
    #[serde(rename = "exactOut")]
    ExactOut,
}

impl Order {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Order::V2DutchOrder(order) => order.encode_inner(),
            Order::PriorityOrder(order) => order.encode_inner(),
            Order::V3DutchOrder(order) => order.encode_inner(),
        }
    }

    pub fn trade_type(&self) -> TradeType {
        match self {
            Order::V2DutchOrder(order) => {
                if order.baseOutputs.iter().any(|o| o.startAmount == o.endAmount) {
                    TradeType::ExactOut
                } else {
                    TradeType::ExactIn
                }
            }
            Order::PriorityOrder(order) => {
                if order.outputs.iter().any(|o| o.mpsPerPriorityFeeWei == U256::from(0)) {
                    TradeType::ExactOut
                } else {
                    TradeType::ExactIn
                }
            }
            Order::V3DutchOrder(order) => {
                if order.baseOutputs.iter().any(
                    |o| o.curve.relativeAmounts.len() == 0 ||
                    o.curve.relativeAmounts.iter().all(|&x| x.eq(&I256::ZERO))
                ) {
                    TradeType::ExactOut
                } else {
                    TradeType::ExactIn
                }
            }
        }
    }

    pub fn is_exact_output(&self) -> bool {
        matches!(self.trade_type(), TradeType::ExactOut)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResolvedInput {
    pub token: String,
    pub amount: U256,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResolvedOutput {
    pub token: String,
    pub amount: U256,
    pub recipient: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResolvedOrder {
    pub input: ResolvedInput,
    pub outputs: Vec<ResolvedOutput>,
}

#[derive(Debug)]
pub enum OrderResolution {
    Resolved(ResolvedOrder),
    Expired,
    Invalid,
    NotFillableYet(ResolvedOrder),
}

impl V2DutchOrder {
    pub fn decode_inner(order_hex: &[u8], _validate: bool) -> Result<Self, Box<dyn Error>> {
        Ok(V2DutchOrder::abi_decode(order_hex)?)
    }

    pub fn encode_inner(&self) -> Vec<u8> {
        V2DutchOrder::abi_encode(self)
    }

    pub fn resolve(&self, timestamp: u64) -> OrderResolution {
        let timestamp = U256::from(timestamp);

        if self.info.deadline.lt(&timestamp) {
            return OrderResolution::Expired;
        };

        // resolve over the decay curve
        // TODO: apply cosigner logic

        let input = ResolvedInput {
            token: self.baseInput.token.to_string(),
            amount: resolve_decay(
                timestamp,
                self.cosignerData.decayStartTime,
                self.cosignerData.decayEndTime,
                self.baseInput.startAmount,
                self.baseInput.endAmount,
            ),
        };

        let outputs: Result<Vec<ResolvedOutput>> = self
            .baseOutputs
            .iter()
            .map(|output| {
                let mut amount = resolve_decay(
                    timestamp,
                    self.cosignerData.decayStartTime,
                    self.cosignerData.decayEndTime,
                    output.startAmount,
                    output.endAmount,
                );

                // add exclusivity override to amount
                if self.cosignerData.decayStartTime.gt(&timestamp) && !self.cosignerData.exclusiveFiller.is_zero() {
                    let exclusivity = self.cosignerData.exclusivityOverrideBps.checked_add(BPS).ok_or(anyhow::Error::msg("Overflow in exclusivity calculation"))?;
                    let exclusivity = exclusivity.checked_mul(amount).ok_or(anyhow::Error::msg("Overflow in exclusivity calculation"))?;
                    amount = exclusivity.checked_div(BPS).ok_or(anyhow::Error::msg("Division by zero in exclusivity calculation"))?;
                };

                Ok(ResolvedOutput {
                    token: output.token.to_string(),
                    amount,
                    recipient: output.recipient.to_string(),
                })
            })
            .collect();

        match outputs {
            Ok(resolved_outputs) => OrderResolution::Resolved(ResolvedOrder { input, outputs: resolved_outputs }),
            Err(_) => OrderResolution::Invalid
        }
    }
}

// Estimates the target block timestamp based on the current block timestamp and average block time
pub fn projected_target_block_ms(current_block: u64, target_block: u64, block_timestamp: u64, block_time_ms: u64) -> U256 {
    let blocks_until_target = U256::from(target_block).saturating_sub(U256::from(current_block));
    let time_until_target_ms = blocks_until_target.saturating_mul(U256::from(block_time_ms));
    U256::from(block_timestamp).saturating_mul(U256::from(1000)).saturating_add(time_until_target_ms)
}

impl PriorityOrder {
    pub fn decode_inner(order_hex: &[u8], _validate: bool) -> Result<Self, Box<dyn Error>> {
        Ok(PriorityOrder::abi_decode(order_hex)?)
    }

    pub fn encode_inner(&self) -> Vec<u8> {
        PriorityOrder::abi_encode(self)
    }

    pub fn resolve(&self, block_number: u64, block_timestamp: u64, block_time_ms: u64, priority_fee: U256, min_block_percentage_buffer: u64) -> OrderResolution {
        let block_time = block_time_ms / 1000;
        let next_block_timestamp = U256::from(block_timestamp) + U256::from(block_time);

        let input = self.input.scale(priority_fee);
        let outputs = self
            .outputs
            .iter()
            .map(|output| output.scale(priority_fee))
            .collect();

        let min_start_block = std::cmp::min(self.cosignerData.auctionTargetBlock, self.auctionStartBlock);

        let current_block = U256::from(block_number);
        if self.info.deadline.lt(&next_block_timestamp) || current_block >= min_start_block {
            return OrderResolution::Expired;
        };
        
        // If current timestamp is > BLOCK_TIME away from target
        // then not yet fillable
        let target_block_ms = projected_target_block_ms(
            block_number,
            min_start_block.try_into().unwrap(),
            block_timestamp,
            block_time_ms
        );
        let time_buffer_ms = block_time_ms * min_block_percentage_buffer / 100;
        if U256::from(current_timestamp_ms() + time_buffer_ms).lt(&target_block_ms) {
            return OrderResolution::NotFillableYet(ResolvedOrder { input, outputs });
        }

        OrderResolution::Resolved(ResolvedOrder { input, outputs })
    }
}

impl PriorityInput {
    pub fn scale(&self, priority_fee: U256) -> ResolvedInput {
        let amount = self.amount.wrapping_mul(U256::from(MPS).wrapping_add(priority_fee.wrapping_mul(self.mpsPerPriorityFeeWei))).wrapping_div(U256::from(MPS));
        ResolvedInput {
            token: self.token.to_string(),
            amount,
        }
    }
}

impl PriorityOutput {
    pub fn scale(&self, priority_fee: U256) -> ResolvedOutput {
        let amount = self.amount.wrapping_mul(U256::from(MPS).saturating_sub(priority_fee.wrapping_mul(self.mpsPerPriorityFeeWei))).wrapping_div(U256::from(MPS));
        ResolvedOutput {
            token: self.token.to_string(),
            amount,
            recipient: self.recipient.to_string(),
        }
    }
}

impl V3DutchOrder {
    pub fn decode_inner(order_hex: &[u8], _validate: bool) -> Result<Self, Box<dyn Error>> {
        Ok(V3DutchOrder::abi_decode(order_hex)?)
    }

    pub fn encode_inner(&self) -> Vec<u8> {
        V3DutchOrder::abi_encode(self)
    }

    pub fn resolve(&self, block_number: u64, timestamp: u64) -> OrderResolution {
        let timestamp = U256::from(timestamp);

        if self.info.deadline.lt(&timestamp) {
            return OrderResolution::Expired;
        };

        // resolve over the decay curve
        let input = ResolvedInput {
            token: self.baseInput.token.to_string(),
            amount: match self.baseInput.curve.decay(
                self.baseInput.startAmount,
                self.cosignerData.decayStartBlock,
                U256::from(block_number),
                U256::from(0),
                self.baseInput.maxAmount,
                NonlinearDutchDecay::v3_linear_input_decay
            ) {
                Ok(amount) => amount,
                Err(_) => return OrderResolution::Invalid,
            },
        };

        let outputs: Result<Vec<ResolvedOutput>> = self
            .baseOutputs
            .iter()
            .map(|output| {
                let mut amount = output.curve.decay(
                    output.startAmount,
                    self.cosignerData.decayStartBlock,
                    U256::from(block_number),
                    output.minAmount,
                    U256::MAX,
                    NonlinearDutchDecay::v3_linear_output_decay
                )?;
                
                // add exclusivity override to amount if before decay start block
                if self.cosignerData.decayStartBlock.gt(&U256::from(block_number)) && !self.cosignerData.exclusiveFiller.is_zero() {
                    let exclusivity = self.cosignerData.exclusivityOverrideBps.checked_add(BPS).ok_or(anyhow::Error::msg("Overflow in exclusivity calculation"))?;
                    let exclusivity = exclusivity.checked_mul(amount).ok_or(anyhow::Error::msg("Overflow in exclusivity calculation"))?;
                    amount = exclusivity.checked_div(BPS).ok_or(anyhow::Error::msg("Division by zero in exclusivity calculation"))?;
                };

                Ok(ResolvedOutput {
                    token: output.token.to_string(),
                    amount,
                    recipient: output.recipient.to_string(),
                })
            })
            .collect();

        match outputs {
            Ok(resolved_outputs) => OrderResolution::Resolved(ResolvedOrder { input, outputs: resolved_outputs }),
            Err(_) => OrderResolution::Invalid,
        }
    }
}

fn resolve_decay(
    at_time: U256,
    start_time: U256,
    end_time: U256,
    start_amount: U256,
    end_amount: U256,
) -> U256 {
    if end_time.le(&at_time) {
        return end_amount;
    }

    if at_time.le(&start_time) {
        return start_amount;
    }

    if end_time.eq(&start_time) {
        return start_amount;
    }

    if start_amount.eq(&end_amount) {
        return start_amount;
    }

    let duration = end_time.saturating_sub(start_time);
    let elapsed = at_time.saturating_sub(start_time);

    if start_amount.gt(&end_amount) {
        // decaying downward
        let decay = start_amount
            .saturating_sub(end_amount)
            .saturating_mul(elapsed)
            .checked_div(duration)
            .unwrap_or(U256::ZERO);
        return start_amount.saturating_sub(decay);
    } else {
        // decaying upward
        let decay = end_amount
            .saturating_sub(start_amount)
            .saturating_mul(elapsed)
            .checked_div(duration)
            .unwrap_or(U256::ZERO);
        return start_amount.saturating_add(decay);
    }
}

impl NonlinearDutchDecay {

    pub fn decay(
        &self,
        start_amount: U256,
        decay_start_block: U256,
        block_numberish: U256,
        min_amount: U256,
        max_amount: U256,
        decay_func: fn(U256, U256, U256, I256, I256) -> Result<I256>
    ) -> Result<U256> {
        // Check for invalid decay curve
        if self.relativeAmounts.len() > PACKED_UINT16_ARRAY_LENGTH {
            return Err(anyhow::anyhow!("Invalid decay curve"));
        }

        // Handle current block before decay or no decay
        if decay_start_block >= block_numberish || self.relativeAmounts.is_empty() {
            return Ok(start_amount.clamp(min_amount, max_amount));
        }

        // Cap block_delta to u16::MAX to prevent overflow
        let block_delta: u16 = u16::try_from(
            (block_numberish - decay_start_block).min(U256::from(u16::MAX))
        )?;

        let (start_point, end_point, rel_start_amount, rel_end_amount) = 
            self.locate_curve_position(block_delta)?;

        // Calculate decay of only the relative amounts
        let curve_delta = (decay_func)(
            U256::from(start_point),
            U256::from(end_point),
            U256::from(block_delta),
            rel_start_amount,
            rel_end_amount,
        )?;

        // Apply curve_delta to start_amount and bound the result
        let result = if curve_delta.is_negative() {
            start_amount.saturating_add(curve_delta.abs().try_into()?)
        } else {
            start_amount.saturating_sub(curve_delta.try_into()?)
        };

        Ok(result.clamp(min_amount, max_amount))
    }

    /// Returns the linear interpolation between two points for input decay
    ///
    /// # Arguments
    ///
    /// * `start_point` - The start of the decay
    /// * `end_point` - The end of the decay
    /// * `current_point` - The current position in the decay
    /// * `start_amount` - The amount at the start of the decay
    /// * `end_amount` - The amount at the end of the decay
    ///
    /// # Returns
    ///
    /// The interpolated amount as an I256
    pub fn v3_linear_input_decay(
        start_point: U256,
        end_point: U256,
        current_point: U256,
        start_amount: I256,
        end_amount: I256,
    ) -> Result<I256> {
        if current_point >= end_point {
            return Ok(end_amount);
        }
        let elapsed = current_point.saturating_sub(start_point);
        let duration = end_point.saturating_sub(start_point);
        let delta: I256;

        // Because start_amount + delta is subtracted from the original amount,
        // we want to maximize start_amount + delta to favor the swapper
        if end_amount < start_amount {
            delta = -(I256::try_from(
                U256::try_from(start_amount.checked_sub(end_amount)
                    .ok_or_else(|| anyhow::anyhow!("Underflow in start_amount - end_amount"))?)?
                    .mul_div_down(elapsed, duration)
                    .map_err(|e| anyhow::anyhow!("MulDivDown error: {}", e))?
            )?);
        } else {
            delta = I256::try_from(
                U256::try_from(end_amount.checked_sub(start_amount)
                .ok_or_else(|| anyhow::anyhow!("Underflow in end_amount - start_amount"))?)?
                .mul_div_up(elapsed, duration)
                .map_err(|e| anyhow::anyhow!("MulDivUp error: {}", e))?
            )?;
        }

        Ok(start_amount.saturating_add(delta))
    }

    /// Returns the linear interpolation between two points for output decay
    ///
    /// # Arguments
    ///
    /// * `start_point` - The start of the decay
    /// * `end_point` - The end of the decay
    /// * `current_point` - The current position in the decay
    /// * `start_amount` - The amount at the start of the decay
    /// * `end_amount` - The amount at the end of the decay
    ///
    /// # Returns
    ///
    /// The interpolated amount as an I256
    pub fn v3_linear_output_decay(
        start_point: U256,
        end_point: U256,
        current_point: U256,
        start_amount: I256,
        end_amount: I256,
    ) -> Result<I256> {
        if current_point >= end_point {
            return Ok(end_amount);
        }
        let elapsed = current_point.saturating_sub(start_point);
        let duration = end_point.saturating_sub(start_point);
        let delta: I256;

        // For outputs, we want to minimize start_amount + delta to favor the swapper
        if end_amount < start_amount {
            delta = -(I256::try_from(
                U256::try_from(start_amount.checked_sub(end_amount)
                    .ok_or_else(|| anyhow::anyhow!("Underflow in start_amount - end_amount"))?)?
                    .mul_div_up(elapsed, duration)
                    .map_err(|e| anyhow::anyhow!("MulDivUp error: {}", e))?
            )?);
        } else {
            delta = I256::try_from(
                U256::try_from(end_amount.checked_sub(start_amount)
                .ok_or_else(|| anyhow::anyhow!("Underflow in end_amount - start_amount"))?)?
                .mul_div_down(elapsed, duration)
                .map_err(|e| anyhow::anyhow!("MulDivDown error: {}", e))?
            )?;
        }

        Ok(start_amount.saturating_add(delta))
    }

    /// Locates the position on the decay curve based on the current block
    fn locate_curve_position(&self, current_relative_block: u16) -> Result<(u16, u16, I256, I256)> {
        // Position is before the start of the curve
        if Self::get_element(self.relativeBlocks, 0)? >= current_relative_block {
            return Ok((0, Self::get_element(self.relativeBlocks, 0)?, I256::ZERO, self.relativeAmounts[0]));
        }
        let last_curve_index = self.relativeAmounts.len() - 1;
        for i in 1..=last_curve_index {
            if Self::get_element(self.relativeBlocks, i)? >= current_relative_block {
                return Ok(
                    (
                        Self::get_element(self.relativeBlocks, i - 1)?,
                        Self::get_element(self.relativeBlocks, i)?,
                        self.relativeAmounts[i - 1],
                        self.relativeAmounts[i],
                    )
                );
            }
        }

        Ok(
            (
                Self::get_element(self.relativeBlocks, last_curve_index)?,
                Self::get_element(self.relativeBlocks, last_curve_index)?,
                self.relativeAmounts[last_curve_index],
                self.relativeAmounts[last_curve_index],
            )
        )
    }

    /// Convert a u16 array into a single Uint<256, 4> value
    /// 
    /// This function packs up to 16 u16 values into a single Uint<256, 4>.
    /// Each u16 value occupies 16 bits in the resulting Uint.
    /// 
    /// # Arguments
    /// 
    /// * `input_array` - A slice of u16 values to be packed
    /// 
    /// # Returns
    /// 
    /// * `Result<Uint<256, 4>>` - The packed Uint value or an error
    pub fn to_uint16_array(input_array: &[u16]) -> Result<U256> {
        if input_array.len() > PACKED_UINT16_ARRAY_LENGTH {
            return Err(anyhow::Error::msg("Invalid array length"));
        }

        let mut packed_data = U256::ZERO;

        for (i, &value) in input_array.iter().enumerate() {
            let shifted_value = U256::from(value) << (i * 16);
            packed_data |= shifted_value;
        }

        Ok(packed_data)
    }

    
    /// Retrieve the nth uint16 value from a packed uint256
    fn get_element(packed_data: U256, n: usize) -> Result<u16> {
        if n >= PACKED_UINT16_ARRAY_LENGTH {
            return Err(anyhow::Error::msg("IndexOutOfBounds"));
        }
        
        let shift_amount = n * 16;
        let masked_value = (packed_data >> shift_amount) & U256::from(0xFFFF);
        let result = u16::try_from(masked_value)?;
        Ok(result)
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    const DECAY_FUNCTIONS: [fn(U256, U256, U256, I256, I256) -> Result<I256>; 2] = [
        NonlinearDutchDecay::v3_linear_input_decay,
        NonlinearDutchDecay::v3_linear_output_decay
    ];

    #[test]
    fn test_decay_after_end_time() {
        let start_time = U256::from(1);
        let end_time = U256::from(10);
        let start_amount = U256::from(100000);
        let end_amount = U256::from(100000000);

        let at_time = U256::from(11);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, end_amount);
    }

    #[test]
    fn test_decay_at_end_time() {
        let start_time = U256::from(1);
        let end_time = U256::from(10);
        let start_amount = U256::from(100000);
        let end_amount = U256::from(100000000);

        let at_time = U256::from(10);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, end_amount);
    }

    #[test]
    fn test_decay_before_start_time() {
        let start_time = U256::from(10);
        let end_time = U256::from(100);
        let start_amount = U256::from(100000);
        let end_amount = U256::from(100000000);

        let at_time = U256::from(5);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, start_amount);
    }

    #[test]
    fn test_decay_at_start_time() {
        let start_time = U256::from(10);
        let end_time = U256::from(100);
        let start_amount = U256::from(100000);
        let end_amount = U256::from(100000000);

        let at_time = U256::from(10);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, start_amount);
    }

    #[test]
    fn test_upwards_decay() {
        let start_time = U256::from(10);
        let end_time = U256::from(20);
        let start_amount = U256::from(100000);
        let end_amount = U256::from(200000);

        let at_time = U256::from(15);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, U256::from(150000));
    }

    #[test]
    fn test_downwards_decay() {
        let start_time = U256::from(10);
        let end_time = U256::from(20);
        let start_amount = U256::from(200000);
        let end_amount = U256::from(100000);

        let at_time = U256::from(15);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, U256::from(150000));
    }

    #[test]
    fn test_nonlinear_decay_before_start() {
        let decay = NonlinearDutchDecay {
            relativeBlocks: NonlinearDutchDecay::to_uint16_array(&vec![
                100,
                200,
                300,
                400,
                500,
            ]).unwrap(),
            relativeAmounts: vec![
                I256::try_from(1000).unwrap(),
                I256::try_from(800).unwrap(),
                I256::try_from(600).unwrap(),
                I256::try_from(400).unwrap(),
                I256::try_from(200).unwrap(),
            ],
        };

        let start_block = U256::from(1000);
        let current_block = U256::from(999);
        let start_amount = U256::from(1000);
        let min_amount = U256::from(0);
        let max_amount = U256::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );
            assert_eq!(result.unwrap(), start_amount);
        }
    }

    #[test]
    fn test_nonlinear_decay_at_start() {
        let decay = NonlinearDutchDecay {
            relativeBlocks: NonlinearDutchDecay::to_uint16_array(&vec![
                100,
                200,
                300,
                400,
                500,
            ]).unwrap(),
            relativeAmounts: vec![
                I256::try_from(1000).unwrap(),
                I256::try_from(800).unwrap(),
                I256::try_from(600).unwrap(),
                I256::try_from(400).unwrap(),
                I256::try_from(200).unwrap(),
            ],
        };

        let start_block = U256::from(1000);
        let current_block = U256::from(1000);
        let start_amount = U256::from(1000);
        let min_amount = U256::from(0);
        let max_amount = U256::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );

            assert_eq!(result.unwrap(), U256::from(1000));
        }
    }

    #[test]
    fn test_nonlinear_decay_midway() {
        let decay = NonlinearDutchDecay {
            relativeBlocks: NonlinearDutchDecay::to_uint16_array(&vec![
                100,
                200,
                300,
                400,
                500,
            ]).unwrap(),
            relativeAmounts: vec![
                I256::try_from(1000).unwrap(),
                I256::try_from(800).unwrap(),
                I256::try_from(600).unwrap(),
                I256::try_from(400).unwrap(),
                I256::try_from(200).unwrap(),
            ],
        };

        let start_block = U256::from(1000);
        let current_block = U256::from(1150);
        let start_amount = U256::from(1000);
        let min_amount = U256::from(0);
        let max_amount = U256::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );

            assert_eq!(result.unwrap(), U256::from(100));
        }
    }

    #[test]
    fn test_nonlinear_decay_at_end() {
        let decay = NonlinearDutchDecay {
            relativeBlocks: NonlinearDutchDecay::to_uint16_array(&vec![
                100,
                200,
                300,
                400,
                500,
            ]).unwrap(),
            relativeAmounts: vec![
                I256::try_from(1000).unwrap(),
                I256::try_from(800).unwrap(),
                I256::try_from(600).unwrap(),
                I256::try_from(400).unwrap(),
                I256::try_from(200).unwrap(),
            ],
        };

        let start_block = U256::from(1000);
        let current_block = U256::from(1500);
        let start_amount = U256::from(1000);
        let min_amount = U256::from(0);
        let max_amount = U256::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );

            assert_eq!(result.unwrap(), U256::from(800));
        }
    }

    #[test]
    fn test_nonlinear_decay_after_end() {
        let decay = NonlinearDutchDecay {
            relativeBlocks: NonlinearDutchDecay::to_uint16_array(&vec![
                100,
                200,
                300,
                400,
                500,
            ]).unwrap(),
            relativeAmounts: vec![
                I256::try_from(1000).unwrap(),
                I256::try_from(800).unwrap(),
                I256::try_from(600).unwrap(),
                I256::try_from(400).unwrap(),
                I256::try_from(200).unwrap(),
            ],
        };

        let start_block = U256::from(1000);
        let current_block = U256::from(1600);
        let start_amount = U256::from(1000);
        let min_amount = U256::from(0);
        let max_amount = U256::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );
        assert_eq!(result.unwrap(), U256::from(800));
        }
    }

    #[test]
    fn test_nonlinear_decay_with_min_amount() {
        let decay = NonlinearDutchDecay {
            relativeBlocks: NonlinearDutchDecay::to_uint16_array(&vec![
                100,
                200,
                300,
                400,
                500,
            ]).unwrap(),
            relativeAmounts: vec![
                I256::try_from(1000).unwrap(),
                I256::try_from(800).unwrap(),
                I256::try_from(600).unwrap(),
                I256::try_from(400).unwrap(),
                I256::try_from(200).unwrap(),
            ],
        };

        let start_block = U256::from(1000);
        let current_block = U256::from(1100);
        let start_amount = U256::from(1000);
        let min_amount = U256::from(300);
        let max_amount = U256::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );

            assert_eq!(result.unwrap(), min_amount);
        }
    }

    #[test]
    fn test_nonlinear_decay_with_max_amount() {
        let decay = NonlinearDutchDecay {
            relativeBlocks: NonlinearDutchDecay::to_uint16_array(&vec![
                100,
                200,
                300,
                400,
                500,
            ]).unwrap(),
            relativeAmounts: vec![
                I256::try_from(1000).unwrap(),
                I256::try_from(800).unwrap(),
                I256::try_from(600).unwrap(),
                I256::try_from(400).unwrap(),
                I256::try_from(200).unwrap(),
            ],
        };

        let start_block = U256::from(1000);
        let current_block = U256::from(1500);
        let start_amount = U256::from(1000);
        let min_amount = U256::from(0);
        let max_amount = U256::from(500);

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );
            assert_eq!(result.unwrap(), max_amount);
        }
    }

    #[test]
    fn test_nonlinear_decay_start_amount_underflow() {
        let decay = NonlinearDutchDecay {
            relativeBlocks: NonlinearDutchDecay::to_uint16_array(&vec![100, 200]).unwrap(),
            relativeAmounts: vec![
                I256::try_from(1000).unwrap(),
                I256::try_from(800).unwrap(),
            ],
        };

        let start_block = U256::from(1000);
        let current_block = U256::from(1100);
        let start_amount = U256::from(500); // Less than max relativeAmount
        let min_amount = U256::from(10);
        let max_amount = U256::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );
            // Cannot fall below min_amount, even upon underflow
            assert_eq!(result.unwrap(), min_amount);
        }
    }

    #[test]
    fn test_nonlinear_decay_start_amount_overflow() {
        let decay = NonlinearDutchDecay {
            relativeBlocks: NonlinearDutchDecay::to_uint16_array(&vec![100, 200]).unwrap(),
            relativeAmounts: vec![
                I256::try_from(-1000).unwrap(),
                I256::try_from(-800).unwrap(),
            ],
        };

        let start_block = U256::from(1000);
        let current_block = U256::from(1100);
        let start_amount = U256::MAX;
        let min_amount = U256::from(10);
        let max_amount = U256::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );
            // Cannot go above max_amount, even upon overflow
            assert_eq!(result.unwrap(), max_amount);
        }
    }

    #[test]
    fn test_nonlinear_decay_relative_blocks_too_long() {
        // Attempt to create the packed relativeBlocks
        let relative_blocks = NonlinearDutchDecay::to_uint16_array(&vec![
            100, 200, 300, 400, 500, 600, 700, 800, 900, 1000,
            1100, 1200, 1300, 1400, 1500, 1600, 1700
        ]);

        // Ensure that to_uint16_array returned an error due to excessive length
        assert!(relative_blocks.is_err());

        // Optionally, you can check the error message
        if let Err(e) = relative_blocks {
            assert_eq!(e.to_string(), "Invalid array length");
        }
    }

    #[test]
    fn test_nonlinear_decay_empty_inputs() {
        let decay_empty = NonlinearDutchDecay {
            relativeBlocks: NonlinearDutchDecay::to_uint16_array(&vec![]).unwrap(),
            relativeAmounts: vec![],
        };
        let start_block = U256::from(1000);
        let current_block = U256::from(1100);
        let start_amount = U256::from(1000);
        let min_amount = U256::from(0);
        let max_amount = U256::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay_empty.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), start_amount);
        }
    }
}
