import { readFileSync } from "node:fs";
import path from "node:path";

import { readdir, readdirSync, readFile } from "fs-extra";
import matter from "gray-matter";
import { MDXRemoteSerializeResult } from "next-mdx-remote";
import { serialize } from "next-mdx-remote/serialize";
import remarkMdx from "remark-mdx";
import remarkMdxDisableExplicitJsx from "remark-mdx-disable-explicit-jsx";
import remarkParse from "remark-parse";
import { unified } from "unified";

import { parseNameFromFileName } from "./client-mdx-util";

type Node = {
  type: string;
};

type Parent = {
  children: Node[];
} & Node;

const isParent = (node: Node): node is Parent => "children" in node;

type Image = {
  type: "image";
  title?: null | string;
  url: string;
  alt?: null | string;
} & Parent;

const isImage = (node: Node): node is Image => node.type === "image";

type ParsedAST = {
  type: "root";
} & Parent;

// Parses the abstract syntax tree of a stringified MDX file
export const parseAST = (mdxFileContent: string) =>
  unified()
    .use(remarkParse)
    .use(remarkMdx)
    .use(remarkMdxDisableExplicitJsx)
    .parse(mdxFileContent) as ParsedAST;

// Recursively returns all the headings in an MDX AST
const getImagesFromParent = (parent: Parent): Image[] => [
  ...parent.children.filter(isImage),
  ...parent.children
    .filter(isParent)
    .flatMap((child) => getImagesFromParent(child)),
];

// Gets all hrefs corresponding to the MDX files in a directory
export const getAllPageHrefs = (params: { folderName: string }): string[] => {
  const { folderName } = params;

  const fileNames = readdirSync(
    path.join(process.cwd(), `src/_pages/${folderName}`),
  );

  return fileNames.map((fileName) => {
    const name = parseNameFromFileName(fileName);

    return `/${folderName}${name === "index" ? "" : `/${name}`}`;
  });
};

// Serializes an MDX file
export const getSerializedPage = async (params: {
  pathToDirectory: string;
  fileNameWithoutIndex: string;
}): Promise<
  [
    MDXRemoteSerializeResult<Record<string, unknown>>,
    Record<string, unknown>,
    // @todo consider matching size here
    Image[],
  ]
> => {
  const { pathToDirectory, fileNameWithoutIndex } = params;

  const fileNames = await readdir(
    path.join(process.cwd(), `src/_pages/${pathToDirectory}`),
  );

  const fileName = fileNames.find((fullFileName) =>
    fullFileName.endsWith(`${fileNameWithoutIndex}.mdx`),
  );

  const source = await readFile(
    path.join(process.cwd(), `src/_pages/${pathToDirectory}/${fileName}`),
  );

  const { content, data } = matter(source);

  const ast = parseAST(content);

  const images = getImagesFromParent(ast);

  const serializedMdx = await serialize(content, {
    // Optionally pass remark/rehype plugins
    mdxOptions: {
      remarkPlugins: [remarkMdxDisableExplicitJsx],
      rehypePlugins: [],
    },
    scope: data,
  });

  return [serializedMdx, data, images];
};

export type Page<DataType extends {}> = {
  fileName: string;
  data: Partial<DataType>;
};

export const getPage = <DataType extends {}>(params: {
  pathToDirectory: string;
  fileName: string;
}): Page<DataType> => {
  const { pathToDirectory, fileName } = params;

  const source = readFileSync(
    path.join(process.cwd(), `src/_pages/${pathToDirectory}/${fileName}`),
  );

  return {
    fileName,
    data: matter(source).data as DataType,
  };
};

export const getAllPages = <DataType extends {}>(
  pathToDirectory: string,
): Page<DataType>[] => {
  const fileNames = readdirSync(
    path.join(process.cwd(), `src/_pages/${pathToDirectory}`),
  );

  return fileNames.map((fileName) =>
    getPage({
      pathToDirectory,
      fileName,
    }),
  );
};
