use gettextrs::gettext;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use i18n_format::i18n_fmt;
use libadwaita as adw;
use libadwaita::prelude::*;
use std::cell::RefCell;
use std::sync::OnceLock;

use crate::leaderboard::{Leaderboard, LeaderboardEntry};

mod imp {
    use super::*;
    use glib::subclass::Signal;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/nacho/mundi/ui/quiz_results_view.ui")]
    pub struct QuizResultsView {
        #[template_child]
        pub score_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub score_caption: TemplateChild<gtk::Label>,
        #[template_child]
        pub time_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub time_caption: TemplateChild<gtk::Label>,
        #[template_child]
        pub retry_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub name_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub name_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub save_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub no_qualify_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub leaderboard_list: TemplateChild<gtk::ListBox>,
        pub save_handler_id: RefCell<Option<glib::SignalHandlerId>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for QuizResultsView {
        const NAME: &'static str = "QuizResultsView";
        type Type = super::QuizResultsView;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for QuizResultsView {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder("retry").build()])
        }

        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj().downgrade();
            self.retry_button.connect_clicked(move |_| {
                if let Some(view) = obj.upgrade() {
                    view.emit_by_name::<()>("retry", &[]);
                }
            });
        }
    }

    impl WidgetImpl for QuizResultsView {}
    impl BoxImpl for QuizResultsView {}
}

glib::wrapper! {
    pub struct QuizResultsView(ObjectSubclass<imp::QuizResultsView>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl QuizResultsView {
    pub fn show_results(
        &self,
        correct: u32,
        total: u32,
        time_secs: u64,
        country_id: &str,
        exercise_id: &str,
    ) {
        let imp = self.imp();

        let pct = if total > 0 {
            (correct as f64 / total as f64 * 100.0).floor()
        } else {
            0.0
        };

        imp.score_label
            .set_text(&i18n_fmt! { i18n_fmt("{}/{}", correct, total) });
        imp.score_caption
            .set_text(&i18n_fmt! { i18n_fmt("Score — {}%", pct) });
        imp.time_label
            .set_text(&format!("{}:{:02}", time_secs / 60, time_secs % 60));
        imp.time_caption.set_text(&gettext("Time"));

        let leaderboard = Leaderboard::load(country_id, exercise_id);
        if leaderboard.qualifies(correct, total, time_secs) {
            imp.name_box.set_visible(true);
            imp.no_qualify_label.set_visible(false);
            imp.name_entry.set_text("");

            if let Some(old_id) = imp.save_handler_id.borrow_mut().take() {
                imp.save_button.disconnect(old_id);
            }
            let view = self.downgrade();
            let cid = country_id.to_string();
            let eid = exercise_id.to_string();
            let id = imp.save_button.connect_clicked(move |_| {
                if let Some(v) = view.upgrade() {
                    v.save_leaderboard_entry(&cid, &eid, correct, total, time_secs);
                }
            });
            *imp.save_handler_id.borrow_mut() = Some(id);
        } else {
            imp.name_box.set_visible(false);
            imp.no_qualify_label
                .set_text(&gettext("Your score didn't make the top 50"));
            imp.no_qualify_label.set_visible(true);
        }
        self.populate_leaderboard(&leaderboard, None);
        self.set_visible(true);
    }

    fn save_leaderboard_entry(
        &self,
        country_id: &str,
        exercise_id: &str,
        score: u32,
        total: u32,
        time_secs: u64,
    ) {
        let imp = self.imp();
        let name = imp.name_entry.text().trim().to_string();
        if name.is_empty() {
            return;
        }

        let mut leaderboard = Leaderboard::load(country_id, exercise_id);
        let entry = LeaderboardEntry {
            name,
            score,
            total,
            time_secs,
        };
        let rank = leaderboard.insert(entry);
        leaderboard.save(country_id, exercise_id);

        imp.name_box.set_visible(false);
        self.populate_leaderboard(&leaderboard, Some(rank));
    }

    fn populate_leaderboard(&self, leaderboard: &Leaderboard, highlight: Option<usize>) {
        let list = &self.imp().leaderboard_list;
        while let Some(child) = list.first_child() {
            list.remove(&child);
        }
        for (i, entry) in leaderboard.entries.iter().enumerate() {
            let pct = if entry.total > 0 {
                (entry.score as f64 / entry.total as f64 * 100.0).floor()
            } else {
                0.0
            };
            let row = adw::ActionRow::builder()
                .title(&entry.name)
                .subtitle(i18n_fmt! {
                    i18n_fmt("{}% — {}:{:02}", pct, entry.time_secs / 60, entry.time_secs % 60)
                })
                .build();
            row.add_prefix(&gtk::Label::new(Some(&format!("{}.", i + 1))));
            if highlight == Some(i) {
                row.add_css_class("accent");
            }
            list.append(&row);
        }
    }
}
