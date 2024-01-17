use crate::reliability::scalingmechanism::ScalingMechanism;
use crate::reliability::scalingtrigger::ScalingTrigger;

pub struct ServiceScalingPolicyDescription {
    mechanism: ScalingMechanism,
    trigger: ScalingTrigger,
}
