use antora_fs::Resource;
use antora_project::component_version::ComponentVersion;
use init_task::InitAssistantResults;
use relative_path::RelativeFile;

// tutorial/pages/index.adoc
pub fn resource_file(
    results: &InitAssistantResults,
    component_version: &ComponentVersion,
) -> (RelativeFile, Resource) {
    (relative_file(), content(results, component_version))
}

fn relative_file() -> RelativeFile {
    "index.adoc"
        .try_into()
        .expect("index.adoc is a valid filename")
}

fn content(results: &InitAssistantResults, component_version: &ComponentVersion) -> Resource {
    let component_name = results.component_name().as_ref();
    let component_title = results.component_title();

    Resource::TextBased(format!(
        r#"= Antora & AsciiDoc Hilfestellung für {component_title}

Dieses Modul unterstützt Sie beim Aufbau Ihrer Dokumentation für die Komponente "{component_name}" (Version {component_version}).

Es enthält Beispiele, Erklärungen und Tipps zu Antora und AsciiDoc.

Sobald Sie Ihre Dokumentation aufgebaut haben, können Sie dieses Modul einfach entfernen:

- Löschen Sie das Verzeichnis `modules/tutorial`
- Entfernen Sie den entsprechenden Navigations-Verweis `modules/tutorial/nav.adoc` in Ihrem _Component-Version-Descriptor_ (`antora.yml`)

Viel Erfolg!

<<<"#
    ))
}
