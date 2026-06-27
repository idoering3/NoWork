import * as SunCalc from 'suncalc';
import { getGeoPosition } from '$lib/misc/position';

type TimeOfDay =
| "night"
| "dawn"
| "day"
| "sunset"
| "dusk";

export async function getSimpleTimeOfDay(
    now: Date,
): Promise<TimeOfDay> {
    const position = await getGeoPosition();

    const t = SunCalc.getTimes(now, position.latitude, position.longitude);

    if (now < t.dawn || now >= t.dusk) {
        return "night";
    }

    if (now < t.sunrise) {
        return "dawn";
    }

    if (now < t.goldenHour) {
        return "day";
    }

    if (now < t.sunset) {
        return "sunset";
    }

    return "dusk";
}
