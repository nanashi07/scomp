-- MySQL 查詢欄位資料
select distinct
    -- 表格名稱
    col.TABLE_NAME,
    -- 欄位名稱
    col.COLUMN_NAME,
    -- 欄位排序
    col.ORDINAL_POSITION,
    -- 欄位預設值
    col.COLUMN_DEFAULT,
    -- 允許空值
    col.IS_NULLABLE,
    -- 資料型別
    col.DATA_TYPE,
    -- 文字長度
    col.CHARACTER_MAXIMUM_LENGTH,
    -- 文字長度
    col.CHARACTER_OCTET_LENGTH,
    -- 數值長度
    col.NUMERIC_PRECISION,
    -- 數值長度
    col.NUMERIC_SCALE,
    -- 欄位型別
    col.COLUMN_TYPE,
    -- 備註
    col.COLUMN_COMMENT,
    -- 主鍵
    case pk.CONSTRAINT_TYPE
        when 'PRIMARY KEY' then
            'YES'
        else
            'NO'
        end PRIMARY_KEY
from information_schema.COLUMNS col
         left join information_schema.TABLE_CONSTRAINTS tc on tc.TABLE_SCHEMA = col.TABLE_SCHEMA
    and tc.TABLE_NAME = col.TABLE_NAME
         left join (
    select tc.TABLE_SCHEMA,
           tc.TABLE_NAME,
           kcu.COLUMN_NAME,
           tc.CONSTRAINT_TYPE
    from information_schema.TABLE_CONSTRAINTS tc
             inner join information_schema.KEY_COLUMN_USAGE kcu on tc.CONSTRAINT_SCHEMA = kcu.CONSTRAINT_SCHEMA
        and tc.CONSTRAINT_NAME = kcu.CONSTRAINT_NAME
        and tc.TABLE_NAME = kcu.TABLE_NAME
    where tc.CONSTRAINT_TYPE = 'PRIMARY KEY') pk on pk.TABLE_SCHEMA = col.TABLE_SCHEMA
    and pk.TABLE_NAME = col.TABLE_NAME
    and pk.COLUMN_NAME = col.COLUMN_NAME
where col.TABLE_SCHEMA = :schema
order by col.TABLE_NAME, col.ORDINAL_POSITION