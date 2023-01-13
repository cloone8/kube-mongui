use std::{thread::{self, sleep}, time::Duration};

use k8s_openapi::{ListResponse, api::batch::v1::{CronJob, JobTemplateSpec}};

use crate::{KubeMonGUI, util::request_util, data::cronjob::{CronJobInfo, CronJobTemplate, CronJobConcurrencyPolicy, CronJobStatus, CronJobContainerInfo}};

fn filter_cronjobs_by_namespace<'a>(cronjobs: &'a [CronJob], namespace: Option<&String>) -> Vec<&'a CronJob> {
    cronjobs.iter()
        .filter(|cj| {
            match namespace {
                Some(ns) => cj.metadata.namespace.is_some() && cj.metadata.namespace.as_ref().unwrap() == ns,
                None => true,
            }
        })
        .collect()
}

fn get_cronjob_info_job_template(job_template: &JobTemplateSpec) -> CronJobTemplate {
    let mut parallelism = None;
    let mut containers = Vec::new();

    if let Some(spec) = &job_template.spec {
        parallelism = spec.parallelism;

        if let Some(pod_specs) = &spec.template.spec {
            let container_info_iter = pod_specs.containers.iter()
                .map(|container| {
                    let name = container.name.clone();
                    let image = container.image.clone();
                    let args = container.args.as_ref().map_or(vec![], |x| x.clone());

                    CronJobContainerInfo {
                        name,
                        image,
                        args
                    }
                });

            containers.extend(container_info_iter);
        }
    }

    CronJobTemplate {
        parallelism,
        containers,
    }
}

fn get_cronjob_info(cronjob: &&CronJob) -> CronJobInfo {
    let name = cronjob.metadata.name.as_ref().unwrap().clone();
    let namespace = cronjob.metadata.namespace.clone();

    let mut schedule = None;
    let mut concurrent = None;
    let mut job_template = None;

    if let Some(spec) = &cronjob.spec {
        schedule = Some(spec.schedule.clone());
        concurrent = spec.concurrency_policy.as_ref().and_then(|cp| CronJobConcurrencyPolicy::try_from(cp.as_str()).ok());
        job_template = Some(get_cronjob_info_job_template(&spec.job_template));
    }

    let status = cronjob.status.as_ref().map(|status| {
        let last_schedule_time = status.last_schedule_time.as_ref().map(|time| time.0);
        let last_successful_time = status.last_successful_time.as_ref().map(|time| time.0);

        CronJobStatus {
            last_schedule_time,
            last_successful_time,
        }
    });

    CronJobInfo {
        name,
        namespace,
        schedule,
        concurrent,
        status,
        job_template
    }
}

pub(crate) fn start(ui_info: &mut KubeMonGUI) -> Result<(), ()> {
    let selected_namespace = ui_info.selected_namespace.clone();
    let cronjobs = ui_info.cronjobs.clone();

    let kube_url = ui_info.k8s_api.get_url();

    let url = format!("{}/apis/batch/v1/cronjobs", kube_url);

    thread::spawn(move || {
        loop {
            let response = request_util::get_response_from_url::<ListResponse<CronJob>>(url.as_str());

            if let Ok(ListResponse::Ok(response)) = response {
                let ns_filtered_cronjobs = filter_cronjobs_by_namespace(&response.items, selected_namespace.lock().as_ref());

                let cronjob_info: Vec<CronJobInfo> = ns_filtered_cronjobs.iter()
                    .map(get_cronjob_info)
                    .collect();

                let mut cronjobs_locked = cronjobs.lock();

                cronjobs_locked.clear();
                cronjobs_locked.extend(cronjob_info);
            }

            sleep(Duration::from_secs(1));
        }
    });

    Ok(())
}
