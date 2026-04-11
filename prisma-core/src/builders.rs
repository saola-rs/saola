/// Generated SelectBuilder pattern
/// This file shows what code generation will produce per model

/// Example: Generated SelectBuilder for User model
pub struct UserSelectBuilder {
    pub fields: Vec<&'static str>,
    pub nested: Vec<(&'static str, Box<dyn std::any::Any>)>,
}

impl UserSelectBuilder {
    #[inline]
    pub fn new() -> Self {
        UserSelectBuilder {
            fields: Vec::new(),
            nested: Vec::new(),
        }
    }

    #[inline]
    pub fn id(&mut self) -> &mut Self {
        self.fields.push("id");
        self
    }

    #[inline]
    pub fn email(&mut self) -> &mut Self {
        self.fields.push("email");
        self
    }

    #[inline]
    pub fn name(&mut self) -> &mut Self {
        self.fields.push("name");
        self
    }

    #[inline]
    pub fn posts<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut PostSelectBuilder),
    {
        let mut post_builder = PostSelectBuilder::new();
        f(&mut post_builder);
        self.nested.push(("posts", Box::new(post_builder)));
        self
    }
}

impl Default for UserSelectBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================

/// Example: Generated SelectBuilder for Post model
pub struct PostSelectBuilder {
    pub fields: Vec<&'static str>,
    pub nested: Vec<(&'static str, Box<dyn std::any::Any>)>,
}

impl PostSelectBuilder {
    #[inline]
    pub fn new() -> Self {
        PostSelectBuilder {
            fields: Vec::new(),
            nested: Vec::new(),
        }
    }

    #[inline]
    pub fn id(&mut self) -> &mut Self {
        self.fields.push("id");
        self
    }

    #[inline]
    pub fn title(&mut self) -> &mut Self {
        self.fields.push("title");
        self
    }

    #[inline]
    pub fn published(&mut self) -> &mut Self {
        self.fields.push("published");
        self
    }
}

impl Default for PostSelectBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================

/// Example: Generated WhereBuilder for User model
pub struct UserWhereBuilder {
    pub conditions: Vec<String>,
    pub relation_filters: Vec<(&'static str, Box<dyn std::any::Any>)>,
}

impl UserWhereBuilder {
    #[inline]
    pub fn new() -> Self {
        UserWhereBuilder {
            conditions: Vec::new(),
            relation_filters: Vec::new(),
        }
    }

    #[inline]
    pub fn email(&mut self) -> EmailFieldFilter<'_> {
        EmailFieldFilter {
            builder: self,
            field: "email",
        }
    }

    #[inline]
    pub fn posts<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut PostWhereBuilder),
    {
        let mut post_filter = PostWhereBuilder::new();
        f(&mut post_filter);
        self.relation_filters.push(("posts", Box::new(post_filter)));
        self
    }

    pub fn conditions_count(&self) -> usize {
        self.conditions.len()
    }
}

impl Default for UserWhereBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================

/// Helper: String field filter builder (chainable)
pub struct EmailFieldFilter<'a> {
    builder: &'a mut UserWhereBuilder,
    field: &'static str,
}

impl<'a> EmailFieldFilter<'a> {
    #[inline]
    pub fn contains(self, value: &'static str) -> &'a mut UserWhereBuilder {
        self.builder.conditions.push(format!("{} CONTAINS {}", self.field, value));
        self.builder
    }

    #[inline]
    pub fn eq(self, value: &'static str) -> &'a mut UserWhereBuilder {
        self.builder.conditions.push(format!("{} = {}", self.field, value));
        self.builder
    }
}

// ============================================================

/// Example: Generated WhereBuilder for Post model
pub struct PostWhereBuilder {
    pub conditions: Vec<String>,
    pub relation_filters: Vec<(&'static str, Box<dyn std::any::Any>)>,
}

impl PostWhereBuilder {
    #[inline]
    pub fn new() -> Self {
        PostWhereBuilder {
            conditions: Vec::new(),
            relation_filters: Vec::new(),
        }
    }

    #[inline]
    pub fn published(&mut self) -> PublishedFieldFilter<'_> {
        PublishedFieldFilter {
            builder: self,
            field: "published",
        }
    }
}

impl Default for PostWhereBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================

/// Helper: Boolean field filter builder
pub struct PublishedFieldFilter<'a> {
    builder: &'a mut PostWhereBuilder,
    field: &'static str,
}

impl<'a> PublishedFieldFilter<'a> {
    #[inline]
    pub fn eq(self, value: bool) -> &'a mut PostWhereBuilder {
        self.builder.conditions.push(format!("{} = {}", self.field, value));
        self.builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_builder_chain() {
        let mut builder = UserSelectBuilder::new();
        builder
            .id()
            .name()
            .email();

        assert_eq!(builder.fields.len(), 3);
    }

    #[test]
    fn test_select_nested() {
        let mut builder = UserSelectBuilder::new();
        builder
            .id()
            .posts(|p| {
                p.id().title();
            });

        assert_eq!(builder.fields.len(), 1);
        assert_eq!(builder.nested.len(), 1);
    }

    #[test]
    fn test_where_builder_chain() {
        let mut builder = UserWhereBuilder::new();
        builder
            .email()
            .contains("@gmail.com");

        assert_eq!(builder.conditions_count(), 1);
    }

    #[test]
    fn test_where_nested() {
        let mut builder = UserWhereBuilder::new();
        builder.posts(|p| {
            p.published().eq(true);
        });

        assert_eq!(builder.relation_filters.len(), 1);
    }
}
