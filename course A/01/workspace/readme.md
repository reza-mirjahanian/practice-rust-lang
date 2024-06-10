
## Creating a Workspace

There are two ways to create a workspace in Rust: using a root package and creating a virtual manifest. In this example, we'll use a virtual manifest to create a workspace.

### Creating a Virtual Manifest

To start, create a new directory called  `blog`  and  `cd`  into it. Then, create a  `Cargo.toml`  file and open it up in your code editor. Add a  `workspace`  section to the file and list the packages in your workspace. Packages in a workspace are called members, so add a  `members`  field to the file.

In this example, we have a blog with three packages:  `blog_api`,  `blog_frontend`, and  `blog_shared`. The  `blog_shared`  package will have a library crate.

### Creating the Packages

Next, open up the terminal and create the packages. First, create the  `blog_api`  package, specifying  `--vcs none`  so that a Git repository isn't automatically created. Then, create the  `blog_web`  package and finally, create the  `blog_shared`  package.

At this point, we have a  `blog`  directory containing a  `Cargo.toml`  file and three packages. This  `Cargo.toml`  file is called a virtual manifest because it defines a workspace rather than a package.

### Building the Workspace

Let's run  `cargo build`  from the root  `blog`  directory. Notice that a  `Cargo.lock`  and a  `target`  directory were generated in the root  `blog`  directory. If we look at  `blog_api`,  `blog_shared`, and  `blog_web`, they do not have a  `target`  directory or  `Cargo.lock`  file.

If we open up the top-level  `target`  directory and click "Debug", we can see that it contains our  `blog_api`  binary, our  `blog_web`  binary, and our  `blog_shared`  library.

Running  `cargo`  commands such as  `build`  or  `test`  from the root of our workspace will build or test every package in the workspace. However, we can target a specific package with the  `-p`  flag. For example, we can build  `blog_api`  by typing  `cargo build -p blog_api`.

### Updating the Packages

Let's update the three packages in our workspace. Starting off with the shared library, we'll add the  `serde`  library as a dependency, which is used for serialization. We'll also turn on the  `derive`  feature so we have access to the  `Serialize`  and  `Deserialize`  derived macros.

Then, we'll open up  `lib.rs`, delete the default test module, and implement a  `Post`  struct and a constructor function. We'll also add a dependency on  `serde`  and derive the  `Serialize`,  `Deserialize`, and  `Debug`  traits for  `Post`.

Now that  `Post`  is defined, let's use it inside our  `blog_api`  and  `blog_frontend`. We'll open up  `Cargo.toml`  inside  `blog_api`  and add  `blog_shared`  as a dependency. Since  `blog_shared`  is a local dependency, we can specify a path to it.

Then, we'll open up  `main.rs`, import  `Post`, create a new  `Post`  in  `main`, and print it out. We can do the same thing for  `blog_web`.