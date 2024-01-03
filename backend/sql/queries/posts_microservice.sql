--! create
INSERT INTO posts
  (owner_id, description)
  VALUES (:owner_id, :description)
		RETURNING id;

--! getPostsOfUser
SELECT id, description, created_at FROM posts
	WHERE owner_id= :owner_id
	LIMIT :pageSize OFFSET :offset;

--! getPosts
SELECT id, owner_id, description, created_at FROM posts
	WHERE id= ANY(:post_ids);