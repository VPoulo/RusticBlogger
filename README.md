# Vaughn Poulo

### 914923269

### Final Project: Rustic Blogger

### Rust Programming

### Fall Term 2023

#### Run Instructions

1. Make sure you are in the **blog_restapi** sub-folder.

2. Use **cargo run** command to start the app service.

3. **click on the link** at the bottom of the terminal to launch the website locally.

** I have already provided a json file with starter blog posts.


#### Write-up

<p>My project is to create a blog website. I wrote a RestAPI as a Rust library that adds, reads, and deletes blog posts from a JSON file. I used this JSON file as a database for the frontend website. The website has two pages, one page that displays all the blog posts, and a second page that is a form to add a new blog post. The restAPI is in the lib.rs file, and the routing and API calls are in the main.rs file.  In the templates folder (within the blog_restapi folder) are the Tera templates I used for the frontend.  I have a template for the landing page, which displays the blog posts, a template for the form page to add a new blog post, and a template for an error page. The JSON file (database) is stored on the main level outside the blog_restapi folder. I have included this in the git repository so that whoever clones this repo has some starting posts to see how the website looks. This posts can be deleted by pushing the delete button below each blog post. I used Rocket to handle the api calls, which worked out very well.</p> 

<p>Using a JSON file for the database was convienent for this mini website, however it isn't a scalable implementation. I think I would change this to use an actual database in the future.  Overall, I am satisfied with the result as this project was a bit of a challenge for me. I am not great at frontend coding / setup but I got everything to work well with a reasonable design.</p>

**There is a warning when compiling the code. It is a warning of a depenency of a crate I'm using. I talked to Bart about it and he said its okay.**

#### License Information
** See [License.txt](https://github.com/VPoulo/RusticBlogger/blob/main/license.txt). 

#### Sources

- https://api.rocket.rs/master/rocket_dyn_templates/

- https://docs.rs/tera/latest/tera/

- https://docs.rs/tera/latest/tera/
