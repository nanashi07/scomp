select
	TABLE_NAME,
	NON_UNIQUE,
	INDEX_NAME,
	SEQ_IN_INDEX,
	COLUMN_NAME
from
	INFORMATION_SCHEMA.STATISTICS
where
	TABLE_SCHEMA = :schema