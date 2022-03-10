CREATE TABLE IF NOT EXISTS namespace (id varchar(32) constraint table_name_pk primary key);

INSERT INTO namespace VALUES ('guard');

CREATE TABLE IF NOT EXISTS namespace_guard (
    role   varchar(32),
    domain varchar(32) default '*',
    object varchar(32),
    action varchar(32),
    constraint namespace_guard_pk primary key (role, domain, object, action)
);

CREATE TABLE IF NOT EXISTS role_guard (
    subject varchar(128),
    domain varchar(32) default '*',
    role varchar(32),
    constraint role_guard_pk primary key (subject, domain, role)
);

CREATE INDEX role_guard_domain_role_index ON role_guard (domain, role);

