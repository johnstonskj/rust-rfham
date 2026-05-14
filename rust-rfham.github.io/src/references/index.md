# References

TBD

## Services

### Callsign Information

Callsign lookup services, which provide information about radio callsigns, such as the
operator's name, location, and other details.

#### Endpoint Variables

* `call`: The callsign to look up.

#### Example Endpoints

* `https://www.qrz.com/db/{call}` (browser)
* `https://xmldata.qrz.com/xml/current/?s={session};callsign={sign}` (API)
* `https://www.radioreference.com/db/ham/callsign/?cs={call}`

### Grid Lookup/Mapping

Locator mapping services, which provides visualization Maidenhead grid locators.

#### Endpoint Variables

* `loc`: The Maidenhead grid locator to visualize.

#### Example Endpoints

* `https://k7fry.com/grid/?qth={loc}`
* `https://lcbsweden.com/www-sm7lcb/maps/qso_map/findlocator/index.html?{loc};`

### Logging

Logging services, which provide a way to log radio contacts and other information.

### Geo Location

IP geolocation services, which provide the geographic location of an IP address.

#### Endpoint Variables

* `ip`: The IP address to geolocate.

### Weather services

which provide weather information for a given location.

### Space weather

Provide information about solar activity and its effects on radio propagation.

### SOTA/POTA/WWFF

Which provide information about summits, parks, and other locations for radio operations.
