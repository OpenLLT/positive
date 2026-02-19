/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 25/12/25
******************************************************************************/

//! Predefined constant values for the `Positive` type.
//!
//! This module provides commonly used numeric constants as `Positive` values,
//! including integers, mathematical constants, and special values.

use crate::Positive;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

// =============================================================================
// Integer Constants (0-10)
// =============================================================================

/// A zero value represented as a `Positive` value.
pub const ZERO: Positive = Positive(Decimal::ZERO);

/// A value of one represented as a `Positive` value.
pub const ONE: Positive = Positive(Decimal::ONE);

/// A value of two represented as a `Positive` value.
pub const TWO: Positive = Positive(Decimal::TWO);

/// A value of three represented as a `Positive` value.
pub const THREE: Positive = Positive(dec!(3));

/// A value of four represented as a `Positive` value.
pub const FOUR: Positive = Positive(dec!(4));

/// A value of five represented as a `Positive` value.
pub const FIVE: Positive = Positive(dec!(5));

/// A value of six represented as a `Positive` value.
pub const SIX: Positive = Positive(dec!(6));

/// A value of seven represented as a `Positive` value.
pub const SEVEN: Positive = Positive(dec!(7));

/// A value of eight represented as a `Positive` value.
pub const EIGHT: Positive = Positive(dec!(8));

/// A value of nine represented as a `Positive` value.
pub const NINE: Positive = Positive(dec!(9));

/// A value of ten represented as a `Positive` value.
pub const TEN: Positive = Positive(Decimal::TEN);

// =============================================================================
// Multiples of 5 (15-95)
// =============================================================================

/// A value of fifteen represented as a `Positive` value.
pub const FIFTEEN: Positive = Positive(dec!(15));

/// A value of twenty represented as a `Positive` value.
pub const TWENTY: Positive = Positive(dec!(20));

/// A value of twenty-five represented as a `Positive` value.
pub const TWENTY_FIVE: Positive = Positive(dec!(25));

/// A value of thirty represented as a `Positive` value.
pub const THIRTY: Positive = Positive(dec!(30));

/// A value of thirty-five represented as a `Positive` value.
pub const THIRTY_FIVE: Positive = Positive(dec!(35));

/// A value of forty represented as a `Positive` value.
pub const FORTY: Positive = Positive(dec!(40));

/// A value of forty-five represented as a `Positive` value.
pub const FORTY_FIVE: Positive = Positive(dec!(45));

/// A value of fifty represented as a `Positive` value.
pub const FIFTY: Positive = Positive(dec!(50));

/// A value of fifty-five represented as a `Positive` value.
pub const FIFTY_FIVE: Positive = Positive(dec!(55));

/// A value of sixty represented as a `Positive` value.
pub const SIXTY: Positive = Positive(dec!(60));

/// A value of sixty-five represented as a `Positive` value.
pub const SIXTY_FIVE: Positive = Positive(dec!(65));

/// A value of seventy represented as a `Positive` value.
pub const SEVENTY: Positive = Positive(dec!(70));

/// A value of seventy-five represented as a `Positive` value.
pub const SEVENTY_FIVE: Positive = Positive(dec!(75));

/// A value of eighty represented as a `Positive` value.
pub const EIGHTY: Positive = Positive(dec!(80));

/// A value of eighty-five represented as a `Positive` value.
pub const EIGHTY_FIVE: Positive = Positive(dec!(85));

/// A value of ninety represented as a `Positive` value.
pub const NINETY: Positive = Positive(dec!(90));

/// A value of ninety-five represented as a `Positive` value.
pub const NINETY_FIVE: Positive = Positive(dec!(95));

// =============================================================================
// Multiples of 100 (100-900)
// =============================================================================

/// A value of one hundred represented as a `Positive` value.
pub const HUNDRED: Positive = Positive(Decimal::ONE_HUNDRED);

/// A value of two hundred represented as a `Positive` value.
pub const TWO_HUNDRED: Positive = Positive(dec!(200));

/// A value of three hundred represented as a `Positive` value.
pub const THREE_HUNDRED: Positive = Positive(dec!(300));

/// A value of four hundred represented as a `Positive` value.
pub const FOUR_HUNDRED: Positive = Positive(dec!(400));

/// A value of five hundred represented as a `Positive` value.
pub const FIVE_HUNDRED: Positive = Positive(dec!(500));

/// A value of six hundred represented as a `Positive` value.
pub const SIX_HUNDRED: Positive = Positive(dec!(600));

/// A value of seven hundred represented as a `Positive` value.
pub const SEVEN_HUNDRED: Positive = Positive(dec!(700));

/// A value of eight hundred represented as a `Positive` value.
pub const EIGHT_HUNDRED: Positive = Positive(dec!(800));

/// A value of nine hundred represented as a `Positive` value.
pub const NINE_HUNDRED: Positive = Positive(dec!(900));

// =============================================================================
// Multiples of 1000 (1000-10000)
// =============================================================================

/// A value of one thousand represented as a `Positive` value.
pub const THOUSAND: Positive = Positive(Decimal::ONE_THOUSAND);

/// A value of two thousand represented as a `Positive` value.
pub const TWO_THOUSAND: Positive = Positive(dec!(2000));

/// A value of three thousand represented as a `Positive` value.
pub const THREE_THOUSAND: Positive = Positive(dec!(3000));

/// A value of four thousand represented as a `Positive` value.
pub const FOUR_THOUSAND: Positive = Positive(dec!(4000));

/// A value of five thousand represented as a `Positive` value.
pub const FIVE_THOUSAND: Positive = Positive(dec!(5000));

/// A value of six thousand represented as a `Positive` value.
pub const SIX_THOUSAND: Positive = Positive(dec!(6000));

/// A value of seven thousand represented as a `Positive` value.
pub const SEVEN_THOUSAND: Positive = Positive(dec!(7000));

/// A value of eight thousand represented as a `Positive` value.
pub const EIGHT_THOUSAND: Positive = Positive(dec!(8000));

/// A value of nine thousand represented as a `Positive` value.
pub const NINE_THOUSAND: Positive = Positive(dec!(9000));

/// A value of ten thousand represented as a `Positive` value.
pub const TEN_THOUSAND: Positive = Positive(dec!(10000));

// =============================================================================
// Mathematical Constants
// =============================================================================

/// The mathematical constant π (pi) represented as a `Positive` value.
/// Approximately 3.14159265358979323846.
pub const PI: Positive = Positive(Decimal::PI);

/// The mathematical constant e (Euler's number) represented as a `Positive` value.
/// Approximately 2.71828182845904523536.
pub const E: Positive = Positive(Decimal::E);

// =============================================================================
// Special Values
// =============================================================================

/// Default epsilon value for approximate comparisons.
/// Used for floating-point tolerance in equality checks.
pub const EPSILON: Decimal = dec!(1e-16);

/// Represents the maximum positive value possible (effectively infinity).
pub const INFINITY: Positive = Positive(Decimal::MAX);

/// Number of days in a year.
pub const DAYS_IN_A_YEAR: Positive = Positive(dec!(365.0));
