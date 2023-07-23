#!/bin/sh -e

sqlx database reset
sqlx migrate run
