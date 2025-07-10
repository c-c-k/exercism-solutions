"""Functions to prevent a nuclear meltdown."""


def is_criticality_balanced(
    temperature: int | float, neutrons_emitted: int | float
) -> bool:
    """Verify criticality is balanced.

    :param temperature: temperature value in kelvin.
    :param neutrons_emitted: number of neutrons emitted per second.
    :return: is criticality balanced?

    A reactor is said to be critical if it satisfies the following conditions:
    - The temperature is less than 800 K.
    - The number of neutrons emitted per second is greater than 500.
    - The product of temperature and neutrons emitted per second is less than 500000.
    """

    if (
        temperature >= 800
        or neutrons_emitted <= 500
        or temperature * neutrons_emitted >= 500000
    ):
        return False
    return True


def reactor_efficiency(
    voltage: int | float,
    current: int | float,
    theoretical_max_power: int | float,
) -> str:
    """Assess reactor efficiency zone.

    :param voltage: voltage value.
    :param current: current value.
    :param theoretical_max_power: power that corresponds to a 100% efficiency.
    :return: one of ('green', 'orange', 'red', or 'black').

    Efficiency can be grouped into 4 bands:

    1. green -> efficiency of 80% or more,
    2. orange -> efficiency of less than 80% but at least 60%,
    3. red -> efficiency below 60%, but still 30% or more,
    4. black ->  less than 30% efficient.

    The percentage value is calculated as
    (generated power/ theoretical max power)*100
    where generated power = voltage * current
    """

    efficiency = (voltage * current / theoretical_max_power) * 100

    if efficiency >= 80:
        return "green"
    elif efficiency >= 60:
        return "orange"
    elif efficiency >= 30:
        return "red"
    else:
        return "black"


def fail_safe(
    temperature: int | float,
    neutrons_produced_per_second: int | float,
    threshold: int | float,
) -> str:
    """Assess and return status code for the reactor.

    :param temperature: value of the temperature in kelvin.
    :param neutrons_produced_per_second: neutron flux.
    :param threshold: threshold for category.
    :return: one of ('LOW', 'NORMAL', 'DANGER').

    1. 'LOW' -> `temperature * neutrons per second` < 90% of `threshold`
    2. 'NORMAL' -> `temperature * neutrons per second` +/- 10% of `threshold`
    3. 'DANGER' -> `temperature * neutrons per second` is not in the above-stated ranges
    """

    discharge = temperature * neutrons_produced_per_second
    if discharge < 0.9 * threshold:
        return "LOW"
    elif discharge <= 1.1 * threshold:
        return "NORMAL"
    else:
        return "DANGER"
