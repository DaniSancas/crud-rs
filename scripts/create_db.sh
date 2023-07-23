#!/bin/sh -e

sqlx database create
sqlx migrate run
