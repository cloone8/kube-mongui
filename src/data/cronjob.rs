use std::fmt::{Formatter, Display};

use chrono::{DateTime, Utc};


#[derive(Debug, Clone)]
pub(crate) struct CronJobInfo {
    pub name: String,
    pub namespace: Option<String>,
    pub schedule: Option<String>,
    pub concurrent: Option<CronJobConcurrencyPolicy>,
    pub status: Option<CronJobStatus>,
    pub job_template: Option<CronJobTemplate>,
}

#[derive(Debug, Clone)]
pub(crate) enum CronJobConcurrencyPolicy {
    Allow,
    Forbid,
    Replace,
}

impl Display for CronJobConcurrencyPolicy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CronJobConcurrencyPolicy::Allow => write!(f, "Allow"),
            CronJobConcurrencyPolicy::Forbid => write!(f, "Forbid"),
            CronJobConcurrencyPolicy::Replace => write!(f, "Replace"),
        }
    }
}

impl TryFrom<&str> for CronJobConcurrencyPolicy {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Allow" => Ok(CronJobConcurrencyPolicy::Allow),
            "Forbid" => Ok(CronJobConcurrencyPolicy::Forbid),
            "Replace" => Ok(CronJobConcurrencyPolicy::Replace),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CronJobStatus {
    pub last_schedule_time: Option<DateTime<Utc>>,
    pub last_successful_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub(crate) struct CronJobTemplate {
    pub parallelism: Option<i32>,
    pub containers: Vec<CronJobContainerInfo>
}

#[derive(Debug, Clone)]
pub(crate) struct CronJobContainerInfo {
    pub name: String,
    pub image: Option<String>,
    pub args: Vec<String>
}
