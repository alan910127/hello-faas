use std::collections::HashMap;

use crate::prelude::*;
use futures::StreamExt;
use shiplift::{
    rep::{Container, ContainerCreateInfo},
    ContainerFilter, ContainerListOptions, ContainerOptions, Docker, PullOptions,
};

pub struct ContainerRepository {
    docker: Docker,
}

impl ContainerRepository {
    pub fn new(docker: Docker) -> Self {
        Self { docker }
    }

    pub async fn pull_image(&self, image: &str) -> Result<()> {
        let opts = PullOptions::builder().image(image).build();
        let mut stream = self.docker.images().pull(&opts);
        while let Some(result) = stream.next().await {
            result.with_context(|| format!("Failed to pull image {}", image))?;
        }
        Ok(())
    }

    pub async fn find_idle(&self) -> Option<String> {
        let opts = ContainerListOptions::builder()
            .filter(vec![
                ContainerFilter::Status("exited".into()),
                ContainerFilter::Label("hello-faas-version".into(), "v1".into()),
                ContainerFilter::LabelName("function-id".into()),
            ])
            .build();

        let containers = self.docker.containers().list(&opts).await.ok()?;

        containers.first().map(|c| c.id.clone())
    }

    pub async fn find_by_function_id(&self, function_id: &str) -> Vec<Container> {
        let opts = ContainerListOptions::builder()
            .filter(vec![
                ContainerFilter::Label("hello-faas-version".into(), "v1".into()),
                ContainerFilter::Label("function-id".into(), function_id.into()),
            ])
            .build();

        self.docker
            .containers()
            .list(&opts)
            .await
            .ok()
            .unwrap_or_default()
    }

    pub async fn create_container(
        &self,
        image: &str,
        function_id: &str,
    ) -> Result<ContainerCreateInfo> {
        let container_name = format!("hello-faas-{}", function_id);
        let opts = ContainerOptions::builder(image)
            .name(&container_name)
            .cpus(0.5)
            .labels(&HashMap::from([
                ("hello-faas-version", "v1"),
                ("function-id", function_id),
            ]))
            .cmd(vec!["/bootstrap"])
            .build();

        self.docker
            .containers()
            .create(&opts)
            .await
            .with_context(|| format!("Failed to create container for function {}", function_id))
    }

    pub async fn start_container(&self, container_id: &str) -> Result<()> {
        self.docker
            .containers()
            .get(container_id)
            .start()
            .await
            .with_context(|| format!("Failed to start container {}", container_id))
    }
}
