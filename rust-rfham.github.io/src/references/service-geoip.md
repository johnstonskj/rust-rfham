# Geo/IP Services

## Provider Responses

| Geo            | GeoIpLookup    | IPinfo free | IPinfo Lite    | IPinfo Core    | Maxmind Country     | Maxmind City              |
| ---------------| -------------- | ----------- | -------------- | -------------- | ------------------- | ------------------------- |
| continent.code | continent_code | n/a         | continent_code | n/a            | continent.code      | continent.code            |
| continent.name | continent_name | n/a         | continent_name | n/a            | continent.names.en  | continent.names.en        |
| country.code   | country_code   | country     | country_code   | country        | country.code        | country.code              |
| country.name   | country_name   | n/a         | country_name   | n/a            | country.names.en    | country.names.en          |
| region         | region         | region      | n/a            | region         | n/a                 | *subdivisions*?           |
| city           | city           | city        | n/a            | city           | n/a                 | city.names.en             |
| timezone.name  | timezone_name  | timezone    | n/a            | timezone       | n/a                 | location.time_zone        |
| postal_code    | postal_code    | postal      | n/a            | postal         | n/a                 | postal.code               |
| geo.latitude   | latitude       | *loc*       | n/a            | *loc*          | n/a                 | location.latitude         |
| geo.longitude  | longitude      | *loc*       | n/a            | *loc*          | n/a                 | location.longitude        |
| geo.accuracy   | n/a            | n/a         | n/a            | n/a            | n/a                 | location.accuracy_radius  |

### Geo Ip Lookup

This uses the publicly accessible API at `https://json.geoiplookup.io/{ip}`.

```json
{
    "ip": "...",
    "isp": "Akamai Technologies, Inc.",
    "org": "Akamai Technologies, Inc.",
    "hostname": "",
    "latitude": 32.814,
    "longitude": -96.9489,
    "postal_code": "",
    "city": "Irving",
    "country_code": "US",
    "country_name": "United States",
    "continent_code": "NA",
    "continent_name": "North America",
    "region": "Texas",
    "district": "",
    "timezone_name": "America/Chicago",
    "connection_type": "Corporate",
    "asn_number": 16625,
    "asn_org": "Akamai Technologies, Inc.",
    "asn": "AS16625 - Akamai Technologies, Inc.",
    "currency_code": "USD",
    "currency_name": "United States Dollar",
    "language_code": "en",
    "language_name": "English",
    "success": true,
    "premium": false
}
```

### IPInfo

Result from `curl https://ipinfo.io/`; legacy free API. See [IPinfo APIs](https://ipinfo.io/developers/ipinfo-api) for comparison.

```json
{
  "ip": "...",
  "hostname": "...",
  "city": "...",
  "region": "...",
  "country": "US",
  "loc": "36.1329,-94.1655",
  "org": "...",
  "postal": "...",
  "timezone": "America/Chicago",
  "readme": "https://ipinfo.io/missingauth"
}
```

Results from the IPInfo Lite API.

```json
{
 ip:"...",
 asn:"AS701",
 as_name:"Verizon Business",
 as_domain:"verizonbusiness.com",
 country_code:"US",
 country:"United States",
 continent_code:"NA",
 continent:"North America"
}
```

Results from the IPInfo Core API.

```json
{
 ip:"...",
 city:"Boston",
 region:"Massachusetts",
 country:"US",
 loc:"42.3584,-71.0598",
 postal:"02151",
 timezone:"America/New_York",
 asn:{
  asn:"AS701",
  name:"Verizon Business",
  domain:"verizonbusiness.com",
  route:"71.243.0.0/17",
  type:"isp"
 },
 is_anycast:false,
 is_mobile:false,
 is_anonymous:false,
 is_satellite:false,
 is_hosting:false
}
```

### Maxmind GeoLite Country

```json
{
  "continent": {
    "code": "NA",
    "geoname_id": 123456,
    "names": { ... }
  },
  "country": {
    "geoname_id": 6252001,
    "is_in_european_union": false,
    "iso_code": "US",
    "names": { ... }
  },
  "registered_country": {
    "geoname_id": 6252001,
    "is_in_european_union": true,
    "iso_code": "US",
    "names": { ... }
  },
  "represented_country": {
    "geoname_id": 6252001,
    "is_in_european_union": true,
    "iso_code": "US",
    "names": { ... },
    "type": "military"
  },
  "traits": {
    "ip_address": "1.2.3.4",
    "is_anycast": true,
    "network": "1.2.3.0/24"
  }
}
```

### Maxmind GeoLite City

```json
{
  "continent": {
    "code": "NA",
    "geoname_id": 123456,
    "names": { ... }
  },
  "country": {
    "geoname_id": 6252001,
    "is_in_european_union": false,
    "iso_code": "US",
    "names": { ... }
  },
  "registered_country": {
    "geoname_id": 6252001,
    "is_in_european_union": true,
    "iso_code": "US",
    "names": { ... }
  },
  "represented_country": {
    "geoname_id": 6252001,
    "is_in_european_union": true,
    "iso_code": "US",
    "names": { ... },
    "type": "military"
  },
  "traits": {
    "ip_address": "1.2.3.4",
    "is_anycast": true,
    "network": "1.2.3.0/24",
    "autonomous_system_number": 1239,
    "autonomous_system_organization": "Linkem IR WiMax Network",
    "connection_type": "Cable/DSL",
    "domain": "example.com",
    "isp": "Linkem spa",
    "mobile_country_code": "310",
    "mobile_network_code": "004",
    "organization": "Linkem IR WiMax Network"
  },
  "city": {
    "geoname_id": 54321,
    "names": { ... }
  },
  "location": {
    "accuracy_radius": 20,
    "latitude": 37.6293,
    "longitude": -122.1163,
    "metro_code": 807,
    "time_zone": "America/Los_Angeles"
  },
  "postal": {
    "code": "90001"
  },
  "subdivisions": [
    {
      "geoname_id": 5332921,
      "iso_code": "CA",
      "names": { ... }
    }
  ]
}
```
