use antora_fs::Resource;
use relative_path::RelativeFile;

// scaffolding/pages/asciidoc-intro.adoc
pub fn relative_file() -> RelativeFile {
    "asciidoc-intro.adoc"
        .try_into()
        .expect("asciidoc-intro.adoc is a valid filename")
}

pub fn content() -> Resource {
    Resource::TextBased(String::from(
        r#"= Einführung in AsciiDoc

AsciiDoc ist eine leicht zu erlernende Auszeichnungssprache für technische Dokumentation.

Hier einige Beispiele:

== Überschriften

[source, adoc]
----
= Hauptüberschrift
== Unterüberschrift
=== Unter-Unterüberschrift
----

== Listen

.Code
[source, asciidoc, opts="linenums,nowrap"]
----
* Ungeordnete Liste
** Verschachtelte Liste

. Geordnete Liste
.. Verschachtelte geordnete Liste
----

* Ungeordnete Liste
** Verschachtelte Liste

. Geordnete Liste
.. Verschachtelte geordnete Liste

== Hervorhebungen

.Code
[source, asciidoc, opts="linenums,nowrap"]
----
*Fett*, _kursiv_, `Monospace`
----

*Fett*, _kursiv_, `Monospace`

== Links

.Code
[source, asciidoc, opts="linenums,nowrap"]
----
https://antora.org[Antora Webseite]
----

https://antora.org[Antora Webseite]

== Tabellen

.Code
[source, asciidoc, opts="linenums,nowrap"]
----
|===
| Spalte 1 | Spalte 2

| Wert 1 | Wert 2
| Wert 3 | Wert 4
|===
----

|===
| Spalte 1 | Spalte 2

| Wert 1 | Wert 2
| Wert 3 | Wert 4
|===

=== Tabelle als Partial einbinden

AsciiDoc erlaubt das Einbinden von Inhalten aus anderen Dateien. Antora erweitert dies um die Nutzung von Resource IDs, die modular und versioniert sind.

Für größere oder mehrfach genutzte Tabellen empfiehlt sich die Auslagerung des Inhalts in eine eigene ``adoc``-Datei in der _partials-family_.

In diesem Beispiel wurde der Inhalt in die Datei ``modules/scaffolding/partials/sample-table.adoc`` ausgelagert und über die _Resource ID_ (relativ zum aktuellen _Modul_) eingebunden:

.AsciiDoc-Code zum Einbinden der Tabelle über einen partial
[source,adoc,opts="linenums,nowrap", subs="attributes,specialchars"]
----
[%header, cols="1,2"]
|===
\include::partial$sample-table.adoc[]
|===
----

.eingebundene Tabelle
[%header, cols="1,2"]
|===
include::scaffolding:partial$sample-table.adoc[]
|===

Vorteile::

* Trennung von Definition und Darstellung der Resource
* Wiederverwendbarkeit des Inhalts an mehreren Stellen (_DRY_-Prinzip)
* Bessere Wartbarkeit und Übersichtlichkeit

<<<

== Bilder

.Code
[source, asciidoc, opts="linenums,nowrap"]
----
.Ein statisches Bild
image::abbildung.svg[]
----

.Ein statisches Bild
image::abbildung.svg[]

== Auf-/Einklappbare Blöcke

Mit Block-Attribut ``collapsible`` können Blöcke _auf-_ und wieder _einklappbar_ gemacht werden (sofern das Zielformat dies unterstützt):

.Code
[source, asciidoc, opts="linenums,nowrap"]
----
[%collapsible]
.ich bin auf- und einklappbar
====
Lange Liste::
mit
* vielen
.. Details
====
----

[%collapsible]
.ich bin auf- und einklappbar
====
Lange Liste::
mit
* vielen
.. Details
====

<<<

Solche Blöcke sind standardmäßig _eingeklappt_ und lassen sich _aufklappen_. +
Man kann sie aber auch umgekehrt als _aufgeklappt_ definieren:

.Code
[source, asciidoc, opts="linenums,nowrap"]
----
[%collapsible%open]
.ich bin ein- und aufklappbar
====
____
Ich bin ja grundsätzlich recht offen ;-)
___
====
----

[%collapsible%open]
.ich bin ein- und aufklappbar
====
____
Ich bin ja grundsätzlich recht offen ;-)
____
====

Weitere Informationen::
* https://docs.asciidoctor.org/asciidoc/latest/
"#,
    ))
}
