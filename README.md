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

## Usage
Build the tool with `cargo build --release`. Tool will be built to "target/release/checkpayments".

`checkpayments ilmodata.json bankdata.nda` will output the differences in payment information and actual payments. This will be printed in .csv like:
```
group_name, missing_payments_eur
Ryhmän Nimi, -15
```
Note that this negative value means that the group has paid too much.

sa. `checkpayments --help` or output into file like so: `checkpayments ilmodata.json bankdata.nda > diff.csv`
