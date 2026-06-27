export async function getGeoPosition(): Promise<GeolocationCoordinates> {
    return await new Promise<GeolocationCoordinates>((resolve, reject) => {
        if (!navigator.geolocation) return reject("Geolocation not supported");
        navigator.geolocation.getCurrentPosition(
        (pos) => resolve(pos.coords),
        (err) => reject(err)
        );
    });
}