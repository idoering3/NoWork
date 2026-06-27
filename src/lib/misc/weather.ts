import { Cloud, CloudDrizzle, CloudFog, CloudHail, CloudLightning, CloudMoon, CloudRain, CloudSnow, CloudSun, Cloudy, Moon, Sun } from '@lucide/svelte';
import { fetchWeatherApi } from 'openmeteo';

export interface WeatherData {
    current: CurrentWeather;
    daily: DailyWeather;
}

export interface CurrentWeather {
    time: Date;
    temperature_2m: number;
    precipitation: number;
    wind_speed_10m: number;
    cloud_cover: number;
    weather_code: number;
    is_day: number;
}

export interface DailyWeather {
    time: Date[];
    temperature_2m_max: Float32Array | null;
    temperature_2m_min: Float32Array | null;
}

export async function getCurrentWeather(lat: number, lon: number): Promise<WeatherData> {
    const params = {
        latitude: lat,
        longitude: lon,
        daily: ["temperature_2m_max", "temperature_2m_min"],
        current: ["temperature_2m", "precipitation", "wind_speed_10m", "cloud_cover", "weather_code", "is_day"],
        timezone: "auto",
        forecast_days: 1,
        wind_speed_unit: "mph",
        temperature_unit: "fahrenheit",
        precipitation_unit: "inch",
    };
    const url = "https://api.open-meteo.com/v1/forecast";
    const responses = await fetchWeatherApi(url, params);

    // Process first location. Add a for-loop for multiple locations or weather models
    const response = responses[0];

    // Attributes for timezone and location
    const latitude = response.latitude();
    const longitude = response.longitude();
    const elevation = response.elevation();
    const timezone = response.timezone();
    const timezoneAbbreviation = response.timezoneAbbreviation();
    const utcOffsetSeconds = response.utcOffsetSeconds();

    const current = response.current()!;
    const daily = response.daily()!;

    // Note: The order of weather variables in the URL query and the indices below need to match!
    const weatherData: WeatherData = {
        current: {
            time: new Date((Number(current.time()) + utcOffsetSeconds) * 1000),
            temperature_2m: current.variables(0)!.value(),
            precipitation: current.variables(1)!.value(),
            wind_speed_10m: current.variables(2)!.value(),
            cloud_cover: current.variables(3)!.value(),
            weather_code: current.variables(4)!.value(),
            is_day: current.variables(5)!.value(),
        },
        daily: {
            time: Array.from(
                { length: (Number(daily.timeEnd()) - Number(daily.time())) / daily.interval() }, 
                (_ , i) => new Date((Number(daily.time()) + i * daily.interval() + utcOffsetSeconds) * 1000)
            ),
            temperature_2m_max: daily.variables(0)!.valuesArray(),
            temperature_2m_min: daily.variables(1)!.valuesArray(),
        },
    };

    // The 'weatherData' object now contains a simple structure, with arrays of datetimes and weather information
    return weatherData;
}



// this function converts weather codes to lucide icons, accounting for day/night
export function getWeatherIcon(weatherCode: number, isDay: boolean) {
    switch (weatherCode) {
        case 0:
            return isDay ? Sun : Moon;
        case 1:
        case 2:
            return isDay ? CloudSun : CloudMoon;
        case 3:
            return Cloud;
        case 45:
        case 48:
            return CloudFog;
        case 51:
        case 53:
        case 55:
        case 56:
        case 57:
            return CloudDrizzle;
        case 61:
        case 63:
        case 65:
        case 66:
        case 67:
        case 80:
        case 81:
        case 82:
            return CloudRain;
        case 71:
        case 73:
        case 75:
        case 77:
        case 85:
        case 86:
            return CloudSnow;
        case 95:
            return CloudLightning;
        case 96:
        case 99:
            return CloudHail;
        default:
            return Cloudy;
    }
}