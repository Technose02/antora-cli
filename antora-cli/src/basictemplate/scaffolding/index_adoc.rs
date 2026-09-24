use antora_fs::Resource;
use antora_project::component_version::ComponentVersion;
use init_template::InitAssistantResults;
use relative_path::RelativeFile;

// scaffolding/pages/index.adoc
pub fn relative_file() -> RelativeFile {
    "index.adoc"
        .try_into()
        .expect("index.adoc is a valid filename")
}

pub fn content(results: &InitAssistantResults, component_version: &ComponentVersion) -> Resource {
    let component_name = results.component_name().as_ref();
    let component_title = results.component_title();

    Resource::TextBased(format!(
        r#"= Antora & AsciiDoc Hilfestellung für {component_title}

Dieses Modul unterstützt Sie beim Aufbau Ihrer Dokumentation für die Komponente "{component_name}" (Version {component_version}).

Es enthält Beispiele, Erklärungen und Tipps zu Antora und AsciiDoc.

Sobald Sie Ihre Dokumentation aufgebaut haben, können Sie dieses Modul einfach entfernen:

- Löschen Sie das Verzeichnis `modules/scaffolding`
- Entfernen Sie den entsprechenden Navigations-Verweis `modules/scaffolding/nav.adoc` in Ihrem _Component-Version-Descriptor_ (`antora.yml`)

Viel Erfolg!

<<<"#
    ))
}
