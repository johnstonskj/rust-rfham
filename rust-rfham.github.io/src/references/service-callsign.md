# Callsign Services

## Qrz.com Notes

```bash
❯ curl 'https://xmldata.qrz.com/xml/current/?username=K7SKJ;password=...;agent=RF-Ham-1.0'
```

```xml
<?xml version="1.0" encoding="utf-8" ?>
<QRZDatabase version="1.36" xmlns="http://xmldata.qrz.com">
  <Session>
    <Key>db5137fe6bb5310fb2cd9b07a1fa04ea</Key>
    <Count>14</Count>
    <SubExp>Mon May  8 20:16:30 2028</SubExp>
    <GMTime>Sat May  9 20:20:10 2026</GMTime>
    <Remark>cpu: 0.013s</Remark>
  </Session>
</QRZDatabase>
```

```bash
❯ curl 'https://xmldata.qrz.com/xml/current/?s=...;callsign=K7SKJ'
```

```xml
<?xml version="1.0" encoding="utf-8" ?>
<QRZDatabase version="1.36" xmlns="http://xmldata.qrz.com">
  <Callsign>
    <call>K7SKJ</call>
    <aliases>KM7ABD</aliases>
    <dxcc>291</dxcc>
    <attn>Simon Johnston</attn>
    <fname>Simon K</fname>
    <name>Johnston</name>
    <addr1>2406 171st Ave SE</addr1>
    <addr2>Bellevue</addr2>
    <state>WA</state>
    <zip>98008-5523</zip>
    <country>United States</country>
    <lat>47.588790</lat>
    <lon>-122.112300</lon>
    <grid>CN87wo</grid>
    <county>King</county>
    <ccode>271</ccode>
    <fips>53033</fips>
    <land>United States</land>
    <efdate>2025-05-28</efdate>
    <expdate>2035-05-28</expdate>
    <class>T</class>
    <codes>HVIE</codes>
    <email>johnstonskj@gmail.com</email>
    <u_views>74</u_views>
    <bio>0</bio>
    <biodate>2025-04-16 18:02:31</biodate>
    <moddate>2025-05-29 12:30:04</moddate>
    <MSA>7600</MSA>
    <AreaCode>425</AreaCode>
    <TimeZone>Pacific</TimeZone>
    <GMTOffset>-8</GMTOffset>
    <DST>Y</DST>
    <lotw>1</lotw>
    <geoloc>geocode</geoloc>
    <name_fmt>Simon K Johnston</name_fmt>
    <serial>2665177</serial>
  </Callsign>
  <Session>
    <Key>db5137fe6bb5310fb2cd9b07a1fa04ea</Key>
    <Count>15</Count>
    <SubExp>Mon May  8 20:16:30 2028</SubExp>
    <GMTime>Sat May  9 20:20:59 2026</GMTime>
    <Remark>cpu: 0.081s</Remark>
  </Session>
</QRZDatabase>
```

| Group    | Field          | Type        | Rq? | K7SKJ (KM7ABD)          | IR3ORCO             | SP9JP                         |
| -------- | -------------- | ----------- | --- | ----------------------- | ------------------- | ----------------------------- |
| Key      | serial         | String      |  Y  | 2665177                 | 2713763             | 2008366                       |
|          | user           | Callsign    |     |                         |                     |                               |
| Sign     | call           | Callsign    |  Y  | 7SKJ                    | IR3ORCO             | SP9JP                         |
|          | xref           | Callsign    |     | (KM7ABD)                |                     |                               |
|          | aliases        | Callsign..  |     | KM7ABD                  |                     | SP7RJA                        |
|          | p_call         | Callsign    |     |                         |                     |                               |
| DX       | dxcc           | u32         |  Y  | 291                     | 248                 | 269                           |
|          | ccode          | u32         |  Y  | 271                     | 128                 | 205                           |
|          | land           | String      |  Y  | United States           | Italy               | Poland                        |
| Name     | fname          | String      |  Y  | Simon K                 | Ari                 | Jurek                         |
|          | name           | String      |  Y  | Johnston                | Udine               | Polczynski                    |
|          | nickname       | String      |     |                         |                     |                               |
|          | name_fmt       | String      |  Y  | Simon K Johnston        | Ari Udine           | Jurek Polczynski              |
| Address  | attn           | String      |     | Simon Johnston          |                     |                               |
|          | addr1          | String      |     | 2406 171st Ave SE       | via Diaz 58         | e-mail me to get full address |
|          | addr2          | String      |     | Bellevue                | UDINE               | Krakow                        |
|          | *state*        | String      |  U  | WA                      |                     |                               |
|          | *fips*         | String      |  U  | 53033                   |                     |                               |
|          | *county*       | String      |  U  | King                    |                     |                               |
|          | zip            | String      |     | 98008-5523              | 33100               |                               |
|          | country        | String      |  Y  | United States           | Italy               | Poland                        |
| Location | lat            | Latitude    |     | 47.588790               | 46.061667           | 50.041964                     |
|          | long           | Longitude   |     | -122.112300             | 13.206667           | 19.918086                     |
|          | grid           | Locator     |     | CN87wo                  | JN66ob              | JO90xb                        |
|          | cqzone         | u32         |     |                         | 15                  | 15                            |
|          | ituzone        | u32         |     |                         | 28                  | 28                            |
|          | geoloc         | enum        |  Y  | geocode                 | user                | user                          |
|          | *MSA*          | u32         |  U  | 7600                    |                     |                               |
|          | *AreaCode*     | u32         |  U  | 425                     |                     |                               |
|          | iota           | String      |     |                         |                     |                               |
| License  | *efdate*       | Date        |  U  | 2025-05-28              |                     |                               |
|          | *expdate*      | Date.       |  U  | 2035-05-28              |                     |                               |
|          | class          | String      |     | T                       |                     | A                             |
|          | *codes*        | String      |  U  | HVIE                    |                     |                               |
| QSL      | qslmgr         | String      |     |                         |                     | LoTW preffered, ...           |
|          | eqsl           | 0/1         |     |                         |                     | 1                             |
|          | mqsl           | 0/1         |     |                         |                     | 1                             |
|          | lotw           | 0/1         |     | 1                       |                     | 1                             |
| Contact  | email          | String      |     | <johnstonskj@gmail.com> |                     | <sp9jp@polczynski.com>        |
| Personal | u_views        | String      |     | 74                      | 121407              | 68024                         |
|          | bio            | u32         |     | 0                       | 1121                | 7556                          |
|          | biodate        | DateTime    |     | 2025-04-16 18:02:31     | 2026-04-24 10:33:55 | 2025-04-02 15:04:45           |
|          | moddate        | DateTime    |     | 2025-05-29 12:30:04     | 2026-03-16 21:40:10 | 2019-06-29 09:42:48           |
|          | image          | Url.        |     |                         | url...              | url...                        |
|          | imageinfo      | String      |     |                         | 355:551:251165      | 212:323:31074                 |
|          | born           | u32         |     |                         |                     | 1971                          |
| TimeZone | *TimeZone*     | String      |  U  | Pacific                 |                     |                               |
|          | GMTOffset      | String      |     | -8                      |                     |                               |
|          | DST            | Y/N         |     | Y                       |                     |                               |

```xml
<?xml version="1.0" ?> 
<QRZDatabase version="1.34">
  <Session>
    <Error>Not found: g1srdd</Error> 
    <Key>1232u4eaf13b8336d61982c1fd1099c9a38ac</Key> 
    <GMTime>Sun Nov 16 05:07:14 2003</GMTime> 
  </Session>
</QRZDatabase>
```
