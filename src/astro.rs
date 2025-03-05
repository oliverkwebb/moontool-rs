// Adapted from "moontool.c" by John Walker
//
// Quoting from the original:
//
//   The algorithms used in this program to calculate the positions Sun and
//   Moon as seen from the Earth are given in the book "Practical Astronomy
//   With  Your  Calculator"  by  Peter  Duffett-Smith,   Second   Edition,
//   Cambridge University Press, 1981.  Ignore the word "Calculator" in the
//   title;  this  is  an  essential  reference  if  you're  interested  in
//   developing  software  which  calculates  planetary  positions, orbits,
//   eclipses, and  the  like.   If  you're  interested  in  pursuing  such
//   programming, you should also obtain:
//
//     "Astronomical  Formulae for Calculators" by Jean Meeus, Third Edition,
//     Willmann-Bell, 1985.  A must-have.
//
//     "Planetary  Programs  and  Tables  from  -4000  to  +2800"  by  Pierre
//     Bretagnon  and Jean-Louis Simon, Willmann-Bell, 1986.  If you want the
//     utmost  (outside  of  JPL)  accuracy  for  the  planets,  it's   here.
//
//     "Celestial BASIC" by Eric Burgess, Revised Edition, Sybex, 1985.  Very
//     cookbook oriented, and many of the algorithms are hard to dig  out  of
//     the turgid BASIC code, but you'll probably want it anyway.
//
// See http://www.fourmilab.ch/moontool/

/// 1980 January 0.0
const EPOCH: f64 = 2444238.5;
/// Ecliptic longitude of the Sun at epoch 1980.0
const ELONGE: f64 = 278.833540;
/// Ecliptic longitude of the Sun at perigee
const ELONGP: f64 = 282.596403;
/// Eccentricity of Earth's orbit
const ECCENT: f64 = 0.016718;
/// Synodic month (new Moon to new Moon)
const SYNMONTH: f64 = 29.53058868;

fn fixangle(a: f64) -> f64 {
    a - 360.0 * (a / 360.0).floor()
}

/// Solve the equation of Kepler
fn kepler(mut m: f64, ecc: f64) -> f64 {
    m = m.to_radians();
    let mut e = m;
    let mut delta: f64;

    delta = e - ecc * e.sin() - m;
    e -= delta / (1.0 - ecc * e.cos());
    while delta.abs() > 1E-6 {
        delta = e - ecc * e.sin() - m;
        e -= delta / (1.0 - ecc * e.cos());
    }
    return e;
}

pub struct MoonState {
    pub precent: f64,
    pub pphase: f64,
    pub mage: f64,
}

/// PHASE  --  Calculate phase of moon as a fraction:
///
///	The argument is the time for which the phase is requested,
///	expressed as a Julian date and fraction.  Returns the terminator
///	phase angle as a percentage of a full circle (i.e., 0 to 1),
///	and stores into pointer arguments the illuminated fraction of
///      the Moon's disc, the Moon's age in days and fraction, the
///	distance of the Moon from the centre of the Earth, and the
///	angular diameter subtended by the Moon as seen by an observer
///	at the centre of the Earth.
///
/// pphase:		Illuminated fraction
/// mage:		Age of moon in days
pub fn phase(pdate: f64) -> MoonState {
    let mut ec;

    /* Calculation of the Sun's position */
    let day = pdate - EPOCH; /* Date within epoch */
    let m = fixangle(fixangle((360.0 / 365.2422) * day) + ELONGE - ELONGP); /* Convert from perigee co-ordinates to epoch 1980.0 */
    ec = kepler(m, ECCENT); /* Solve equation of Kepler */
    ec = ((1.0 + ECCENT) / (1.0 - ECCENT)).sqrt() * (ec / 2.0).tan();
    ec = 2.0 * ec.atan().to_degrees(); /* True anomaly */
    let lambdasun = fixangle(ec + ELONGP); /* Sun's geocentric ecliptic longitude */

    /* Moon's mean longitude */
    let ml = fixangle(13.1763966 * day + 64.975464); /* Moon's mean lonigitude at the epoch */

    /* Moon's mean anomaly */
    let mm = fixangle(ml - 0.1114041 * day - 349.383063); /* 349:  Mean longitude of the perigee at the epoch */

    /* Evection */
    let ev = 1.2739 * (2.0 * (ml - lambdasun) - mm).to_radians().sin();

    /* Annual equation */
    let ae = 0.1858 * m.to_radians().sin();

    /* Corrected anomaly */
    let mmp = mm + ev - ae - (0.37 * m.to_radians().sin());

    /* Corrected longitude */
    let lp =
        ml + ev + (6.2886 * mmp.to_radians().sin()) - ae + (0.214 * (2.0 * mmp).to_radians().sin());

    /* True longitude */
    let lpp = lp + (0.6583 * (2.0 * (lp - lambdasun).to_radians().sin()));

    /* Age of the Moon in degrees */
    let moonage = lpp - lambdasun;

    MoonState {
        pphase: (1.0 - moonage.to_radians().cos()) / 2.0,
        mage: SYNMONTH * (fixangle(moonage) / 360.0),
        precent: fixangle(moonage) / 360.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_moon() {
        assert_eq!(phase(2460740.165938).pphase, 0.3940552678252821);
    }
}
