mod since_v0_8_0;

use extension::{KeyValueStoreDelegate, WorktreeDelegate};
use gpui::BackgroundExecutor;
use language::LanguageName;
use lsp::LanguageServerName;
use release_channel::ReleaseChannel;

use super::{WasmState, wasm_engine};
use anyhow::{Context as _, Result};
use semver::Version;
use since_v0_8_0 as latest;
use std::{ops::RangeInclusive, sync::Arc};
use wasmtime::{
    Store,
    component::{Component, Linker, Resource},
};

#[cfg(test)]
pub use latest::CodeLabelSpanLiteral;
pub use latest::{
    CodeLabel, CodeLabelSpan, Command, Range, SlashCommand,
    zed::extension::lsp::{
        Completion, CompletionKind, CompletionLabelDetails, InsertTextFormat, Symbol, SymbolKind,
    },
    zed::extension::slash_command::{SlashCommandArgumentCompletion, SlashCommandOutput},
};

pub fn new_linker(
    executor: &BackgroundExecutor,
    f: impl FnOnce(&mut Linker<WasmState>) -> Result<()>,
) -> Linker<WasmState> {
    let mut linker = Linker::new(&wasm_engine(executor));
    wasmtime_wasi::p2::add_to_linker_async(&mut linker).unwrap();
    f(&mut linker).unwrap();
    linker
}

pub fn is_supported_wasm_api_version(release_channel: ReleaseChannel, version: Version) -> bool {
    wasm_api_version_range(release_channel).contains(&version)
}

#[inline(always)]
pub fn wasm_api_version_range(release_channel: ReleaseChannel) -> RangeInclusive<Version> {
    let _ = release_channel;
    latest::MIN_VERSION..=latest::MAX_VERSION
}

pub enum Extension {
    V0_8_0(since_v0_8_0::Extension),
}

impl Extension {
    pub async fn instantiate_async(
        executor: &BackgroundExecutor,
        store: &mut Store<WasmState>,
        release_channel: ReleaseChannel,
        version: Version,
        component: &Component,
    ) -> Result<Self> {
        let _ = release_channel;
        anyhow::ensure!(
            version >= latest::MIN_VERSION,
            "Wasm extension API versions before v0.8.0 are not supported in lite builds"
        );

        let extension =
            latest::Extension::instantiate_async(store, component, latest::linker(executor))
                .await
                .context("failed to instantiate wasm extension")?;
        Ok(Self::V0_8_0(extension))
    }

    pub async fn call_init_extension(&self, store: &mut Store<WasmState>) -> Result<()> {
        match self {
            Extension::V0_8_0(ext) => ext.call_init_extension(store).await,
        }
    }

    pub async fn call_language_server_command(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        language_name: &LanguageName,
        resource: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> Result<Result<Command, String>> {
        let _ = language_name;
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_language_server_command(store, &language_server_id.0, resource)
                    .await
            }
        }
    }

    pub async fn call_language_server_initialization_options(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        language_name: &LanguageName,
        resource: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> Result<Result<Option<String>, String>> {
        let _ = language_name;
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_language_server_initialization_options(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
        }
    }

    pub async fn call_language_server_workspace_configuration(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        resource: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> Result<Result<Option<String>, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_language_server_workspace_configuration(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
        }
    }

    pub async fn call_language_server_initialization_options_schema(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        resource: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> Result<Option<String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_language_server_initialization_options_schema(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
        }
    }

    pub async fn call_language_server_workspace_configuration_schema(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        resource: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> Result<Option<String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_language_server_workspace_configuration_schema(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
        }
    }

    pub async fn call_language_server_additional_initialization_options(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        target_language_server_id: &LanguageServerName,
        resource: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> Result<Result<Option<String>, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_language_server_additional_initialization_options(
                    store,
                    &language_server_id.0,
                    &target_language_server_id.0,
                    resource,
                )
                .await
            }
        }
    }

    pub async fn call_language_server_additional_workspace_configuration(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        target_language_server_id: &LanguageServerName,
        resource: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> Result<Result<Option<String>, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_language_server_additional_workspace_configuration(
                    store,
                    &language_server_id.0,
                    &target_language_server_id.0,
                    resource,
                )
                .await
            }
        }
    }

    pub async fn call_labels_for_completions(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        completions: Vec<latest::Completion>,
    ) -> Result<Result<Vec<Option<CodeLabel>>, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_labels_for_completions(store, &language_server_id.0, &completions)
                    .await
            }
        }
    }

    pub async fn call_labels_for_symbols(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        symbols: Vec<latest::Symbol>,
    ) -> Result<Result<Vec<Option<CodeLabel>>, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_labels_for_symbols(store, &language_server_id.0, &symbols)
                    .await
            }
        }
    }

    pub async fn call_complete_slash_command_argument(
        &self,
        store: &mut Store<WasmState>,
        command: &SlashCommand,
        arguments: &[String],
    ) -> Result<Result<Vec<SlashCommandArgumentCompletion>, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_complete_slash_command_argument(store, command, arguments)
                    .await
            }
        }
    }

    pub async fn call_run_slash_command(
        &self,
        store: &mut Store<WasmState>,
        command: &SlashCommand,
        arguments: &[String],
        resource: Option<Resource<Arc<dyn WorktreeDelegate>>>,
    ) -> Result<Result<SlashCommandOutput, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_run_slash_command(store, command, arguments, resource)
                    .await
            }
        }
    }

    pub async fn call_suggest_docs_packages(
        &self,
        store: &mut Store<WasmState>,
        provider: &str,
    ) -> Result<Result<Vec<String>, String>> {
        match self {
            Extension::V0_8_0(ext) => ext.call_suggest_docs_packages(store, provider).await,
        }
    }

    pub async fn call_index_docs(
        &self,
        store: &mut Store<WasmState>,
        provider: &str,
        package_name: &str,
        kv_store: Resource<Arc<dyn KeyValueStoreDelegate>>,
    ) -> Result<Result<(), String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_index_docs(store, provider, package_name, kv_store)
                    .await
            }
        }
    }
}

trait ToWasmtimeResult<T> {
    fn to_wasmtime_result(self) -> wasmtime::Result<Result<T, String>>;
}

impl<T> ToWasmtimeResult<T> for Result<T> {
    fn to_wasmtime_result(self) -> wasmtime::Result<Result<T, String>> {
        Ok(self.map_err(|error| format!("{error:?}")))
    }
}
