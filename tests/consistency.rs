#[cfg(test)]
mod tests {
    use deta::{Deta, Item};

    #[tokio::test]
    async fn put_get() -> anyhow::Result<()> {
        let deta = Deta::new()?;
        let deta = deta.base("test");

        deta.delete("Hello").await?;

        let item = Item::new_with_key("Hello", 5usize);

        deta.insert(item).await?;
        let Item { value, .. }: Item<usize> = deta.get("Hello").await?;

        assert_eq!(value, 5usize);

        Ok(())
    }

    #[tokio::test]
    async fn put_get_many() -> anyhow::Result<()> {
        use futures::stream::{self, StreamExt, TryStreamExt};

        let deta = Deta::new()?;
        let deta = deta.base("test_many");

        let stream = stream::iter(0..10usize);

        stream
            .map(|x| Ok((x, x, deta.clone())))
            .try_for_each_concurrent(10, |(x, y, deta)| async move {
                let item = Item::new_with_key(x, y);
                deta.delete(x).await?;
                deta.insert(item).await?;
                let Item { value, .. }: Item<usize> = deta.get(x).await?;
                assert_eq!(value, y);

                deta::Result::Ok(())
            })
            .await?;

        Ok(())
    }
}
