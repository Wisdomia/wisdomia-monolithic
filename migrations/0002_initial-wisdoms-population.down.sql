BEGIN TRANSACTION;

DELETE FROM wisdoms WHERE id IN (1, 2, 3);

COMMIT;