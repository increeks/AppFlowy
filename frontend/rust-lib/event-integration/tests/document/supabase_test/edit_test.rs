use std::time::Duration;

use event_integration::assert_document_data_equal;
use flowy_document2::entities::DocumentSyncStatePB;

use crate::document::supabase_test::helper::FlowySupabaseDocumentTest;
use crate::util::receive_with_timeout;

#[tokio::test]
async fn supabase_document_edit_sync_test() {
  if let Some(test) = FlowySupabaseDocumentTest::new().await {
    let view = test.create_document().await;
    let document_id = view.id.clone();

    let cloned_test = test.clone();
    let cloned_document_id = document_id.clone();
    tokio::spawn(async move {
      cloned_test
        .insert_document_text(&cloned_document_id, "hello world", 0)
        .await;
    });

    // wait all update are send to the remote
    let mut rx = test
      .notification_sender
      .subscribe_with_condition::<DocumentSyncStatePB, _>(&document_id, |pb| pb.is_finish);
    receive_with_timeout(&mut rx, Duration::from_secs(30))
      .await
      .unwrap();

    let document_data = test.get_document_data(&document_id).await;
    let update = test.get_document_update(&document_id).await;
    assert_document_data_equal(&update, &document_id, document_data);
  }
}

#[tokio::test]
async fn supabase_document_edit_sync_test2() {
  if let Some(test) = FlowySupabaseDocumentTest::new().await {
    let view = test.create_document().await;
    let document_id = view.id.clone();

    for i in 0..10 {
      test
        .insert_document_text(&document_id, "hello world", i)
        .await;
    }

    // wait all update are send to the remote
    let mut rx = test
      .notification_sender
      .subscribe_with_condition::<DocumentSyncStatePB, _>(&document_id, |pb| pb.is_finish);
    receive_with_timeout(&mut rx, Duration::from_secs(30))
      .await
      .unwrap();

    let document_data = test.get_document_data(&document_id).await;
    let update = test.get_document_update(&document_id).await;
    assert_document_data_equal(&update, &document_id, document_data);
  }
}
