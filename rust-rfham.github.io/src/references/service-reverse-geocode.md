# Reverse Geo-Coding

```xml
<?xml version="1.0" encoding="UTF-8" ?>
<reversegeocode
  timestamp="Thu, 14 May 2026 22:52:22 +00:00"
  attribution="Data © OpenStreetMap contributors, ODbL 1.0. http://osm.org/copyright"
  querystring="lat=47.979&amp;lon=-122.2021&amp;format=xml">
  <result 
    place_id="388919907" 
    osm_type="node" 
    osm_id="12347384589" 
    ref="Sports Physical Therapy" 
    lat="47.9789401" 
    lon="-122.2020953" 
    boundingbox="47.9788901,47.9789901,-122.2021453,-122.2020453" 
    place_rank="30" 
    address_rank="30">Sports Physical Therapy, 2000, Hewitt Avenue, Port Gardner, Everett, Snohomish County, Washington, 98201, United States
  </result>
  <addressparts>
    <healthcare>Sports Physical Therapy</healthcare>
    <house_number>2000</house_number>
    <road>Hewitt Avenue</road>
    <suburb>Port Gardner</suburb>
    <city>Everett</city>
    <county>Snohomish County</county>
    <state>Washington</state>
    <ISO3166-2-lvl4>US-WA</ISO3166-2-lvl4>
    <postcode>98201</postcode>
    <country>United States</country>
    <country_code>us</country_code>
  </addressparts>
</reversegeocode>
```

| Element           | US | UK | ID | CN |
| ----------------- | -- | -- | -- | -- |
| house number      | Y  | Y  | ?  | ?  |
| road              | Y  | Y  | Y  | Y  |
| suburb            | Y  | Y  | N  | Y  |
| city              | Y  | Y  | N  | Y  |
| village           | N  | N  | Y  | N  |
| county            | Y  | N  | Y  | N  |
| state             | Y  | Y  | Y  | Y  |
| village           | N  | N  | Y  | N  |
| postcode          | Y  | Y  | Y  | Y  |
| country           | Y  | Y  | Y  | Y  |
| country code      | Y  | Y  | Y  | Y  |
| ISO3166-2 lvl 3   | N  | N  | Y  | N  |
| ISO3166-2 lvl 4   | Y  | Y  | Y  | Y  |
| ISO3166-2 lvl 6   | N  | Y  | N  | N  |

```xml
<addressparts>
  <shop>Primark</shop>
  <house_number>129-135</house_number>
  <road>Commercial Road</road>
  <suburb>Somers Town</suburb>
  <city>Portsmouth</city>
  <ISO3166-2-lvl6>GB-POR</ISO3166-2-lvl6>
  <state>England</state>
  <ISO3166-2-lvl4>GB-ENG</ISO3166-2-lvl4>
  <postcode>PO1 1BU</postcode>
  <country>United Kingdom</country>
  <country_code>gb</country_code>
</addressparts>    
```

```xml
<addressparts>
  <road>Jalan Raya</road>
  <village>Waha</village>
  <county>Wakatobi</county>
  <state>Sulawesi Tenggara</state>
  <ISO3166-2-lvl4>ID-SG</ISO3166-2-lvl4>
  <region>Sulawesi</region>
  <ISO3166-2-lvl3>ID-SL</ISO3166-2-lvl3>
  <postcode>93797</postcode>
  <country>Indonesia</country>
  <country_code>id</country_code>
</addressparts>
```

```xml
<addressparts>
  <historic>御制平定西藏碑</historic>
  <road>北京中路</road>
  <suburb>རྗེ་འབུམ་སྒང་ཁྲོམ་གཞུང་། 吉崩岗街道</suburb>
  <city>城关区 ཁྲིན་ཀོན་ཆུས།</city>
  <state>西藏自治区 བོད་རང་སྐྱོང་ལྗོངས།</state>
  <ISO3166-2-lvl4>CN-XZ</ISO3166-2-lvl4>
  <postcode>850000</postcode>
  <country>中国</country>
  <country_code>cn</country_code>
</addressparts>
```
