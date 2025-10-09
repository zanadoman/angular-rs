use askama::Template;
use time::OffsetDateTime;

#[derive(Template)]
#[template(path = "report_mail.html")]
pub struct ReportMail<'a> {
  pub username: &'a str,
  pub action: &'a str,
  pub timestamp: &'a OffsetDateTime,
}
