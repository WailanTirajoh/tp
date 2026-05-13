-- Drop audit_logs table
DROP INDEX IF EXISTS idx_audit_logs_created_at;

DROP INDEX IF EXISTS idx_audit_logs_table_record;

DROP TABLE IF EXISTS audit_logs;