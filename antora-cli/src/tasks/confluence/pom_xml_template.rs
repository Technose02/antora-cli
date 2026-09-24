use antora_project::{antora_configuration::ConfluenceConfig, antora_playbook::AttributeValue};
use std::collections::HashMap;
use xml::{EmitterConfig, common::XmlVersion, writer::XmlEvent};

pub const CONFLUENCE_PAT_ENV_VARIABLE: &str = "CONFLUENCE_PAT";

fn internal_contents(
    confluence_config: &ConfluenceConfig,
    attributes: Option<&HashMap<String, AttributeValue>>,
) -> Result<String, String> {
    let mut buf = Vec::new();

    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(std::io::Cursor::new(&mut buf));

    writer
        .write(XmlEvent::StartDocument {
            version: XmlVersion::Version10,
            encoding: Some("utf-8"),
            standalone: Some(true),
        })
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(
            XmlEvent::start_element("project")
                .default_ns("http://maven.apache.org/POM/4.0.0")
                .ns("xsi", "http://www.w3.org/2001/XMLSchema-instance")
                .attr("xsi:schemaLocation", "http://maven.apache.org/POM/4.0.0"),
        )
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::start_element("modelVersion"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters("4.0.0"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;

    writer
        .write(XmlEvent::start_element("groupId"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters("DUMMY"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;

    writer
        .write(XmlEvent::start_element("artifactId"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters("DUMMY"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;

    writer
        .write(XmlEvent::start_element("version"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters("DUMMY"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;

    writer
        .write(XmlEvent::start_element("name"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters("DUMMY"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;

    writer
        .write(XmlEvent::start_element("description"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters("DUMMY"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::start_element("build"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::start_element("plugins"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::start_element("plugin"))
        .map_err(|e| format!("xml-error: {e}"))?;

    writer
        .write(XmlEvent::start_element("groupId"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters(
            "org.sahli.asciidoc.confluence.publisher",
        ))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::start_element("artifactId"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters(
            "asciidoc-confluence-publisher-maven-plugin",
        ))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::start_element("version"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters(
            confluence_config.publisher_version.as_str(),
        ))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::start_element("configuration"))
        .map_err(|e| format!("xml-error: {e}"))?;

    writer
        .write(XmlEvent::start_element("asciidocRootFolder"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters(
            confluence_config.asciidoc_root_folder.as_str(),
        ))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;

    writer
        .write(XmlEvent::start_element("sourceEncoding"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters("UTF-8"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::start_element("rootConfluenceUrl"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters(
            confluence_config.root_confluence_url.as_str(),
        ))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::start_element("skipSslVerification"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters(
            confluence_config.skip_ssl_verification.to_string().as_str(),
        ))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::start_element("maxRequestsPerSecond"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters(
            confluence_config
                .max_requests_per_second
                .to_string()
                .as_str(),
        ))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::start_element("spaceKey"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters(confluence_config.space_key.as_str()))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::start_element("ancestorId"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters(confluence_config.ancestor_id.as_str()))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::start_element("password"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters(
            format!("${{{CONFLUENCE_PAT_ENV_VARIABLE}}}").as_str(),
        ))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;

    if let Some(page_title_prefix) = &confluence_config.page_title_prefix {
        writer
            .write(XmlEvent::start_element("pageTitlePrefix").attr("xml:space", "preserve"))
            .map_err(|e| format!("xml-error: {e}"))?;
        writer
            .write(XmlEvent::Characters(page_title_prefix.as_str()))
            .map_err(|e| format!("xml-error: {e}"))?;
        writer
            .write(XmlEvent::end_element())
            .map_err(|e| format!("xml-error: {e}"))?;
    }

    if let Some(page_title_suffix) = &confluence_config.page_title_suffix {
        writer
            .write(XmlEvent::start_element("pageTitleSuffix").attr("xml:space", "preserve"))
            .map_err(|e| format!("xml-error: {e}"))?;
        writer
            .write(XmlEvent::Characters(page_title_suffix.as_str()))
            .map_err(|e| format!("xml-error: {e}"))?;
        writer
            .write(XmlEvent::end_element())
            .map_err(|e| format!("xml-error: {e}"))?;
    }

    writer
        .write(XmlEvent::start_element("versionMessage"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters(
            format!("Version {}", confluence_config.projectversion).as_str(),
        ))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::start_element("notifyWatchers"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters(
            confluence_config.notify_watchers.to_string().as_str(),
        ))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;

    writer
        .write(XmlEvent::start_element("publishingStrategy"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters(
            confluence_config.publishing_strategy.to_string().as_str(),
        ))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;

    writer
        .write(XmlEvent::start_element("orphanRemovalStrategy"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters(
            confluence_config
                .orphan_removal_strategy
                .to_string()
                .as_str(),
        ))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::start_element("restApiVersion"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters(
            confluence_config.rest_api_version.as_str(),
        ))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;

    writer
        .write(XmlEvent::start_element("attributes"))
        .map_err(|e| format!("xml-error: {e}"))?;

    writer
        .write(XmlEvent::start_element("version"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters(
            confluence_config.projectversion.as_str(),
        ))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;

    writer
        .write(XmlEvent::start_element("confluence-publish"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::Characters("true"))
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;

    if let Some(attributes) = attributes {
        for (k, v) in attributes.iter() {
            let value = match v {
                AttributeValue::Bool(bool_value) => Some(bool_value.to_string()),
                AttributeValue::String(s) => Some(s.clone()),
                _ => None,
            };

            if let Some(value) = value {
                writer
                    .write(XmlEvent::start_element(k.as_str()))
                    .map_err(|e| format!("xml-error: {e}"))?;
                writer
                    .write(XmlEvent::Characters(value.as_str()))
                    .map_err(|e| format!("xml-error: {e}"))?;
                writer
                    .write(XmlEvent::end_element())
                    .map_err(|e| format!("xml-error: {e}"))?;
            }
        }
    }

    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| format!("xml-error: {e}"))?;

    String::from_utf8(buf).map_err(|e| format!("{e}"))
}

pub fn contents(
    confluence_config: &ConfluenceConfig,
    attributes: Option<&HashMap<String, AttributeValue>>,
) -> String {
    match internal_contents(confluence_config, attributes) {
        Ok(contents) => contents,
        Err(e) => panic!("error creating confluence-publish-xml: {e}"),
    }
}
