//! Handler for all the of the set database interactions

use diesel;
use diesel::prelude::*;
use actix_web::{web, Responder};

use crate::models;
