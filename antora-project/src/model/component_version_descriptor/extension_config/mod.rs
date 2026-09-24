use serde::{Deserialize, Serialize};

pub mod collector_extension;

use collector_extension::CollectorExtensionConfig;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ExtensionConfig {
    collector: Option<CollectorExtensionConfig>,
}

#[allow(unused)]
impl ExtensionConfig {
    pub fn collector(mut self, collector: impl Into<CollectorExtensionConfig>) -> Self {
        self.collector = Some(collector.into());
        self
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use collector_extension::clean::{CleanValue, CleanValueArrayEntry, CleanValueMap};
    use collector_extension::run::{
        EnvEntry, RunEnv, RunValue, RunValueArrayBuilder, RunValueArrayEntry, RunValueMap,
    };
    use collector_extension::scan::{
        ScanValue, ScanValueArrayBuilder, ScanValueArrayEntry, ScanValueMap,
    };
    use collector_extension::{CollectorConfigArrayBuilder, CollectorConfigMapBuilder};

    macro_rules! print_and_try_deserialize_extension_config {
        ($config_ref:expr) => {{
            let s = serde_yaml_bw::to_string($config_ref).unwrap();
            println!("{s}");
            _ = serde_yaml_bw::from_str::<ExtensionConfig>(s.as_str()).unwrap();
        }};
    }

    #[test]
    fn clean_tests() {
        let clean_val_1 = CleanValue::from(String::from("build-foo"));
        let clean_val_2 = {
            let mut v = Vec::new();
            v.push(CleanValueArrayEntry::String("build-foo".into()));
            v.push(CleanValueArrayEntry::Map(CleanValueMap::new(String::from(
                "build-foo",
            ))));
            CleanValue::from(v)
        };
        let clean_val_3: CleanValue = CleanValueMap::new(String::from("build-foo")).into();

        let config1 = ExtensionConfig::default().collector(
            CollectorConfigArrayBuilder::default()
                .clean(clean_val_1.clone())
                .clean(clean_val_2.clone())
                .clean(clean_val_3.clone())
                .run(RunValue::String("irrelevant".into()))
                .build(),
        );

        let config2 = ExtensionConfig::default().collector(
            CollectorConfigMapBuilder::default()
                .clean(clean_val_1)
                .run(RunValue::String("irrelevant".into()))
                .build(),
        );

        let config3 = ExtensionConfig::default().collector(
            CollectorConfigMapBuilder::default()
                .clean(clean_val_2)
                .run(RunValue::String("irrelevant".into()))
                .build(),
        );

        let config4 = ExtensionConfig::default().collector(
            CollectorConfigMapBuilder::default()
                .clean(clean_val_3)
                .run(RunValue::String("irrelevant".into()))
                .build(),
        );

        print_and_try_deserialize_extension_config!(&config1);
        print_and_try_deserialize_extension_config!(&config2);
        print_and_try_deserialize_extension_config!(&config3);
        print_and_try_deserialize_extension_config!(&config4);
    }

    #[test]
    fn run_tests() {
        let run_val_1 = RunValue::from(String::from("generate-all-the-files"));
        let run_val_2 = {
            let mut v = Vec::new();
            v.push(RunValueArrayEntry::String("generate-more-files".into()));
            v.push(RunValueArrayEntry::Map(
                RunValueMap::new(String::from("generate-files"))
                    .local(true)
                    .env(RunEnv::Array(vec![EnvEntry::new(
                        "foo".into(),
                        "bar".into(),
                    )])),
            ));
            RunValue::from(v)
        };

        let config1 = ExtensionConfig::default().collector(
            CollectorConfigArrayBuilder::default()
                .clean(CleanValue::String("irrelevant".into()))
                .run(run_val_1.clone())
                .run(run_val_2.clone())
                .build(),
        );

        let config2 = ExtensionConfig::default().collector(
            CollectorConfigMapBuilder::default()
                .clean(CleanValue::String("irrelevant".into()))
                .run(run_val_1)
                .build(),
        );

        print_and_try_deserialize_extension_config!(&config1);
        print_and_try_deserialize_extension_config!(&config2);
    }

    #[test]
    fn scan_tests() {
        let scan_val_1 = ScanValue::from(String::from("build/generated"));
        let scan_val_2 = {
            let mut v = Vec::new();
            v.push(ScanValueArrayEntry::String("build/generated".into()));
            v.push(ScanValueArrayEntry::Map(
                ScanValueMap::new(String::from("build/generated"))
                    .files("**/*.adoc")
                    .clean(true),
            ));
            ScanValue::from(v)
        };

        let config1 = ExtensionConfig::default().collector(
            CollectorConfigArrayBuilder::default()
                .scan(scan_val_1.clone())
                .scan(scan_val_2.clone())
                .build(),
        );

        let config2 = ExtensionConfig::default().collector(
            CollectorConfigMapBuilder::default()
                .scan(scan_val_2)
                .build(),
        );

        print_and_try_deserialize_extension_config!(&config1);
        print_and_try_deserialize_extension_config!(&config2);
    }

    #[test]
    fn structurizr_template_test() {
        let config = ExtensionConfig::default().collector(
             CollectorConfigArrayBuilder::default()
            .clean("doc/modules/ROOT/attachments")
            .run(RunValueArrayBuilder::default()
            .push(RunValueMap::new("uv run asset_generator.py ../shared/constants/technologies.json generated").dir("asset_generation"))
            .push(RunValueMap::new("uv run asset_generator.py ../shared/constants/softwaresystem_tags.json generated").dir("asset_generation"))
            .push(RunValueMap::new("uv run asset_generator.py ../shared/constants/container_tags.json generated").dir("asset_generation"))
            .push(RunValueMap::new("uv run asset_generator.py ../shared/constants/relationship_tags.json generated").dir("asset_generation"))
            .build())
            .scan(ScanValueArrayBuilder::default().push(ScanValueMap::new("template").files("workspace.dsl").into("modules/ROOT/examples/template.dsl"))
            .push(ScanValueMap::new("doc/..").files("confluence_pat.secret.template").into("modules/ROOT/examples/"))
            .push(ScanValueMap::new("doc/..").files("git-credentials.secret.template").into("modules/ROOT/examples/"))
            .push(ScanValueMap::new("doc/..").files("build_and_publish.ps1").into("modules/ROOT/examples/"))
            .push(ScanValueMap::new("doc/..").files("antora-playbook.yml").into("modules/ROOT/examples/"))
            .push(ScanValueMap::new("doc/..").files("export.yml").into("modules/ROOT/examples/"))
            .push(ScanValueMap::new("shared/style").files("*.dsl").into("modules/konventionen_vorgaben/examples/"))
            .push(ScanValueMap::new("shared/constants").files("*.dsl,*.json").into("modules/konventionen_vorgaben/examples/"))
            .push(ScanValueMap::new("asset_generation/generated/").files("*.adoc").into("modules/konventionen_vorgaben/partials/"))
            .push(ScanValueMap::new("asset_generation/generated/").files("*.dsl").into("modules/konventionen_vorgaben/examples/"))
            .build())
        .build(),
    );

        print_and_try_deserialize_extension_config!(&config);
    }
}
