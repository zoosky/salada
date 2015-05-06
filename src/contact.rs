use std::default::Default;
use std::collections::HashSet;
use jmap::method::*;
use jmap::util::Presence::*;

use util::RequestContext;
use db::DbError;

pub trait ContactHandler {
    fn get_contacts(&self, args: &GetRequestArgs)               -> Result<GetResponseArgs,DbError>;
    fn get_contact_updates(&self, args: &GetUpdatesRequestArgs) -> Result<GetUpdatesResponseArgs,DbError>;
    fn set_contacts(&self, args: &SetRequestArgs)               -> Result<SetResponseArgs,DbError>;
}

impl ContactHandler for RequestContext {
    fn get_contacts(&self, args: &GetRequestArgs) -> Result<GetResponseArgs,DbError> {
        // XXX assuming success through here
        let txn = self.db.transaction().unwrap();
        let records = self.db.get_records(self.userid, args.ids.as_option()).unwrap();
        let state = self.db.get_state(self.userid).unwrap();
        txn.commit().unwrap();

        let not_found = match args.ids {
            Absent => None,
            Present(ref ids) => {
                let mut found = HashSet::new();
                for record in records.iter() {
                    found.insert(&record.id);
                }
                let not_found = ids.into_iter().filter(|id| !found.contains(id)).map(|id| id.clone()).collect::<Vec<_>>();
                match not_found.len() {
                    0 => None,
                    _ => Some(not_found),
                }
            }
        };

        let list = match args.properties {
            Absent         => Some(records.iter().map(|ref r| r.to_partial()).collect()),
            Present(ref p) => Some(records.iter().map(|ref r| r.to_filtered_partial(p)).collect()),
        };

        let response = GetResponseArgs {
            state: state,
            list: list,
            not_found: not_found,
        };

        Ok(response)
    }

    fn get_contact_updates(&self, args: &GetUpdatesRequestArgs) -> Result<GetUpdatesResponseArgs,DbError> {
        println!("get_contact_updates: {:?}", args);
        Ok(GetUpdatesResponseArgs::default())
    }

    fn set_contacts(&self, args: &SetRequestArgs) -> Result<SetResponseArgs,DbError> {
        println!("set_contacts: {:?}", args);
        Ok(SetResponseArgs::default())
    }
}
