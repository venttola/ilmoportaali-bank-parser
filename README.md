# Ilmoportaali Bank data parser
Tämän projektin tarkoituksena on tuottaa työkalu., jonka avulla Ilmoportaalista tulevan maksudatan ja Nordean verkkopankin konekielisen tiliotteen vertailu on jatkossa helpompaa.

## Nopea speksi
Syötteet:
- Ilmoportaalista JSON, jossa ryhmän maksutiedot (malli hakemistossa `example_data`).
- Nordeasta muinaisessa formaatissa oleva tiliote (NDA ja PRN), jossa tilitapahtumat.

Haluttu toiminnallisuus:
- Kerätä pankkimaksujen viitenumeroiden perusteella selkokielinen tietorakenne, josta käy ilmi, että minkä verran minkäkin ryhmän olisi tullut suorittaa maksuja ja kuinka paljon maksuja on tullut tilille.

Tulosformaatti:
- TBA. Jotain kivaa ja helposti luettavaa. Vaikka json

Tarkennellaan speksiä sitten kun aivot toimivat paremmin.
